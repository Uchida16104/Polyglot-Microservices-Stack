const std = @import("std");
const Allocator = std.mem.Allocator;

const MAX_OUTPUT: usize = 524288;
const TIMEOUT_SECS: u64 = 60;

const Lang = enum {
    zig,
    mojo,
    dafny,
    fstar,

    fn fromStr(s: []const u8) ?Lang {
        if (std.mem.eql(u8, s, "zig")) return .zig;
        if (std.mem.eql(u8, s, "mojo")) return .mojo;
        if (std.mem.eql(u8, s, "dafny")) return .dafny;
        if (std.mem.eql(u8, s, "fstar") or std.mem.eql(u8, s, "f*")) return .fstar;
        return null;
    }

    fn ext(self: Lang) []const u8 {
        return switch (self) {
            .zig => "zig",
            .mojo => "mojo",
            .dafny => "dfy",
            .fstar => "fst",
        };
    }
};

const ExecResult = struct {
    stdout: []u8,
    stderr: []u8,
    exit_code: i32,
    duration_ms: i64,
    verification_passed: ?bool,
    alloc: Allocator,

    fn deinit(self: ExecResult) void {
        self.alloc.free(self.stdout);
        self.alloc.free(self.stderr);
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const alloc = gpa.allocator();

    const port: u16 = blk: {
        const env = std.process.getEnvVarOwned(alloc, "PORT") catch null;
        defer if (env) |e| alloc.free(e);
        break :blk if (env) |e| std.fmt.parseInt(u16, e, 10) catch 8004 else 8004;
    };

    const addr = std.net.Address.initIp4(.{ 0, 0, 0, 0 }, port);
    var net_server = try addr.listen(.{ .reuse_address = true });
    defer net_server.deinit();

    std.log.info("research-runtime listening on port {d}", .{port});

    var read_buf: [65536]u8 = undefined;

    while (true) {
        const conn = net_server.accept() catch |err| {
            std.log.err("accept: {}", .{err});
            continue;
        };
        var http_srv = std.http.Server.init(conn, &read_buf);
        var req = http_srv.receiveHead() catch |err| {
            std.log.err("receiveHead: {}", .{err});
            conn.stream.close();
            continue;
        };
        route(alloc, &req) catch |err| {
            std.log.err("route: {}", .{err});
        };
        conn.stream.close();
    }
}

fn respond(req: *std.http.Server.Request, status: std.http.Status, body: []const u8) !void {
    try req.respond(body, .{
        .status = status,
        .extra_headers = &.{
            .{ .name = "content-type", .value = "application/json" },
            .{ .name = "access-control-allow-origin", .value = "*" },
        },
    });
}

fn route(alloc: Allocator, req: *std.http.Server.Request) !void {
    const path = req.head.target;
    const method = req.head.method;

    if (std.mem.eql(u8, path, "/health")) {
        try respond(req, .ok, "{\"status\":\"ok\",\"service\":\"research-runtime\"}");
        return;
    }

    if (method != .POST) {
        try respond(req, .method_not_allowed, "{\"message\":\"method not allowed\"}");
        return;
    }

    if (std.mem.eql(u8, path, "/compile") or std.mem.eql(u8, path, "/run")) {
        try handleCompile(alloc, req);
        return;
    }

    try respond(req, .not_found, "{\"message\":\"not found\"}");
}

fn handleCompile(alloc: Allocator, req: *std.http.Server.Request) !void {
    var body_list = std.ArrayList(u8).init(alloc);
    defer body_list.deinit();

    var chunk: [4096]u8 = undefined;
    var reader = req.reader();
    while (true) {
        const n = reader.read(&chunk) catch break;
        if (n == 0) break;
        try body_list.appendSlice(chunk[0..n]);
        if (body_list.items.len > 512 * 1024) break;
    }

    const parsed = std.json.parseFromSlice(std.json.Value, alloc, body_list.items, .{}) catch {
        try respond(req, .bad_request, "{\"message\":\"invalid JSON\"}");
        return;
    };
    defer parsed.deinit();

    const obj = switch (parsed.value) {
        .object => |o| o,
        else => {
            try respond(req, .bad_request, "{\"message\":\"expected JSON object\"}");
            return;
        },
    };

    const language_str = if (obj.get("language")) |v| switch (v) {
        .string => |s| s,
        else => "",
    } else {
        try respond(req, .bad_request, "{\"message\":\"language required\"}");
        return;
    };

    const source_code = if (obj.get("source_code")) |v| switch (v) {
        .string => |s| s,
        else => "",
    } else {
        try respond(req, .bad_request, "{\"message\":\"source_code required\"}");
        return;
    };

    const stdin_opt: ?[]const u8 = if (obj.get("stdin_data")) |v| switch (v) {
        .string => |s| s,
        else => null,
    } else null;

    const flags_opt: ?[]const u8 = if (obj.get("compiler_flags")) |v| switch (v) {
        .string => |s| s,
        else => null,
    } else null;

    const lang = Lang.fromStr(language_str) orelse {
        try respond(req, .bad_request, "{\"message\":\"unsupported language: use zig mojo dafny fstar\"}");
        return;
    };

    const result = executeResearch(alloc, lang, source_code, stdin_opt, flags_opt) catch |err| {
        var eb: [256]u8 = undefined;
        const msg = std.fmt.bufPrint(&eb, "{{\"message\":\"exec error: {s}\"}}", .{@errorName(err)}) catch
            "{\"message\":\"exec error\"}";
        try respond(req, .internal_server_error, msg);
        return;
    };
    defer result.deinit();

    var out = std.ArrayList(u8).init(alloc);
    defer out.deinit();

    const status_str: []const u8 = if (result.exit_code == 0) "completed" else "failed";

    if (result.verification_passed) |vp| {
        try std.json.stringify(.{
            .status = status_str,
            .stdout = result.stdout,
            .stderr = result.stderr,
            .exit_code = result.exit_code,
            .duration_ms = result.duration_ms,
            .verification_passed = vp,
        }, .{}, out.writer());
    } else {
        try std.json.stringify(.{
            .status = status_str,
            .stdout = result.stdout,
            .stderr = result.stderr,
            .exit_code = result.exit_code,
            .duration_ms = result.duration_ms,
        }, .{}, out.writer());
    }

    try respond(req, .ok, out.items);
}

fn executeResearch(
    alloc: Allocator,
    lang: Lang,
    source: []const u8,
    stdin: ?[]const u8,
    flags: ?[]const u8,
) !ExecResult {
    const ts = std.time.milliTimestamp();
    const tmp_dir = try std.fmt.allocPrint(alloc, "/tmp/research-{d}", .{ts});
    defer alloc.free(tmp_dir);

    try std.fs.makeDirAbsolute(tmp_dir);
    defer std.fs.deleteTreeAbsolute(tmp_dir) catch {};

    const src_path = try std.fmt.allocPrint(alloc, "{s}/main.{s}", .{ tmp_dir, lang.ext() });
    defer alloc.free(src_path);

    {
        const f = try std.fs.createFileAbsolute(src_path, .{});
        defer f.close();
        try f.writeAll(source);
    }

    var argv = std.ArrayList([]const u8).init(alloc);
    defer argv.deinit();

    switch (lang) {
        .zig => {
            try argv.appendSlice(&.{ "zig", "run", src_path });
        },
        .mojo => {
            try argv.appendSlice(&.{ "mojo", "run", src_path });
        },
        .dafny => {
            try argv.appendSlice(&.{ "dafny", "verify", src_path });
        },
        .fstar => {
            try argv.appendSlice(&.{ "fstar.exe", src_path });
        },
    }

    if (flags) |f| {
        var iter = std.mem.splitScalar(u8, f, ' ');
        while (iter.next()) |flag| {
            if (flag.len > 0) try argv.append(flag);
        }
    }

    const start_ms = std.time.milliTimestamp();
    const proc_result = try runProcess(alloc, argv.items, stdin, tmp_dir);
    const duration_ms = std.time.milliTimestamp() - start_ms;

    const verification_passed: ?bool = switch (lang) {
        .dafny, .fstar => proc_result.exit_code == 0,
        else => null,
    };

    return ExecResult{
        .stdout = proc_result.stdout,
        .stderr = proc_result.stderr,
        .exit_code = proc_result.exit_code,
        .duration_ms = duration_ms,
        .verification_passed = verification_passed,
        .alloc = alloc,
    };
}

const ProcResult = struct {
    stdout: []u8,
    stderr: []u8,
    exit_code: i32,
};

fn runProcess(alloc: Allocator, argv: []const []const u8, stdin_data: ?[]const u8, cwd: []const u8) !ProcResult {
    var child = std.process.Child.init(argv, alloc);
    child.cwd = cwd;
    child.stdin_behavior = if (stdin_data != null) .Pipe else .Ignore;
    child.stdout_behavior = .Pipe;
    child.stderr_behavior = .Pipe;

    try child.spawn();

    if (stdin_data) |sd| {
        if (child.stdin) |stdin_file| {
            try stdin_file.writeAll(sd);
            stdin_file.close();
            child.stdin = null;
        }
    }

    const stdout = try child.stdout.?.reader().readAllAlloc(alloc, MAX_OUTPUT);
    const stderr = try child.stderr.?.reader().readAllAlloc(alloc, MAX_OUTPUT);

    const term = try child.wait();
    const exit_code: i32 = switch (term) {
        .Exited => |code| @as(i32, @intCast(code)),
        .Signal => |sig| @as(i32, @intCast(sig)) * -1,
        .Stopped => -2,
        .Unknown => -3,
    };

    return ProcResult{ .stdout = stdout, .stderr = stderr, .exit_code = exit_code };
}
