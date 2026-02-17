import asyncio
import os
import tempfile
import shutil
from dataclasses import dataclass
from typing import Optional

EXECUTION_TIMEOUT_SECS = 60


@dataclass
class CompileResult:
    verification_output: Optional[str]
    execution_output: Optional[str]
    error_output: Optional[str]
    exit_code: int


async def _run_command(
    cmd: list[str],
    stdin_data: Optional[str] = None,
    timeout: int = EXECUTION_TIMEOUT_SECS,
) -> tuple[str, str, int]:
    proc = await asyncio.create_subprocess_exec(
        *cmd,
        stdin=asyncio.subprocess.PIPE if stdin_data else asyncio.subprocess.DEVNULL,
        stdout=asyncio.subprocess.PIPE,
        stderr=asyncio.subprocess.PIPE,
    )
    try:
        stdin_bytes = stdin_data.encode() if stdin_data else None
        stdout_bytes, stderr_bytes = await asyncio.wait_for(
            proc.communicate(stdin_bytes), timeout=timeout
        )
    except asyncio.TimeoutError:
        proc.kill()
        await proc.wait()
        return "", f"Timed out after {timeout}s", -1

    return (
        stdout_bytes.decode(errors="replace"),
        stderr_bytes.decode(errors="replace"),
        proc.returncode or 0,
    )


async def run_zig(source_code: str, flags: Optional[str]) -> CompileResult:
    tmp = tempfile.mkdtemp(prefix="zig_")
    try:
        src = os.path.join(tmp, "main.zig")
        binary = os.path.join(tmp, "output")
        with open(src, "w") as f:
            f.write(source_code)

        compile_cmd = ["zig", "build-exe", src, f"-femit-bin={binary}"]
        if flags:
            compile_cmd.extend(flags.split())

        c_out, c_err, c_code = await _run_command(compile_cmd)
        if c_code != 0:
            return CompileResult(
                verification_output=None,
                execution_output=None,
                error_output=c_err or c_out,
                exit_code=c_code,
            )

        e_out, e_err, e_code = await _run_command([binary])
        return CompileResult(
            verification_output=c_out or None,
            execution_output=e_out or None,
            error_output=e_err or None,
            exit_code=e_code,
        )
    finally:
        shutil.rmtree(tmp, ignore_errors=True)


async def run_mojo(source_code: str, flags: Optional[str]) -> CompileResult:
    tmp = tempfile.mkdtemp(prefix="mojo_")
    try:
        src = os.path.join(tmp, "main.mojo")
        with open(src, "w") as f:
            f.write(source_code)

        cmd = ["mojo", "run", src]
        if flags:
            cmd.extend(flags.split())

        stdout, stderr, code = await _run_command(cmd)
        return CompileResult(
            verification_output=None,
            execution_output=stdout or None,
            error_output=stderr or None,
            exit_code=code,
        )
    finally:
        shutil.rmtree(tmp, ignore_errors=True)


async def run_dafny(source_code: str, flags: Optional[str]) -> CompileResult:
    tmp = tempfile.mkdtemp(prefix="dafny_")
    try:
        src = os.path.join(tmp, "main.dfy")
        with open(src, "w") as f:
            f.write(source_code)

        verify_cmd = ["dafny", "verify", src]
        if flags:
            verify_cmd.extend(flags.split())

        v_out, v_err, v_code = await _run_command(verify_cmd)

        if v_code != 0:
            return CompileResult(
                verification_output=v_out or v_err,
                execution_output=None,
                error_output=v_err or None,
                exit_code=v_code,
            )

        run_cmd = ["dafny", "run", src]
        e_out, e_err, e_code = await _run_command(run_cmd)
        return CompileResult(
            verification_output=v_out or None,
            execution_output=e_out or None,
            error_output=e_err or None,
            exit_code=e_code,
        )
    finally:
        shutil.rmtree(tmp, ignore_errors=True)


async def run_fstar(source_code: str, flags: Optional[str]) -> CompileResult:
    tmp = tempfile.mkdtemp(prefix="fstar_")
    try:
        src = os.path.join(tmp, "main.fst")
        with open(src, "w") as f:
            f.write(source_code)

        cmd = ["fstar.exe", src]
        if flags:
            cmd.extend(flags.split())

        stdout, stderr, code = await _run_command(cmd)
        return CompileResult(
            verification_output=stdout or None,
            execution_output=None,
            error_output=stderr or None,
            exit_code=code,
        )
    finally:
        shutil.rmtree(tmp, ignore_errors=True)


async def dispatch(runtime: str, source_code: str, flags: Optional[str]) -> CompileResult:
    handlers = {
        "zig": run_zig,
        "mojo": run_mojo,
        "dafny": run_dafny,
        "fstar": run_fstar,
    }
    handler = handlers.get(runtime.lower())
    if handler is None:
        return CompileResult(
            verification_output=None,
            execution_output=None,
            error_output=f"Unsupported runtime: {runtime}",
            exit_code=1,
        )
    return await handler(source_code, flags)
