namespace BusinessService.Middleware;

public class InternalAuthMiddleware
{
    private readonly RequestDelegate _next;
    private readonly ILogger<InternalAuthMiddleware> _logger;

    public InternalAuthMiddleware(RequestDelegate next, ILogger<InternalAuthMiddleware> logger)
    {
        _next = next;
        _logger = logger;
    }

    public async Task InvokeAsync(HttpContext context)
    {
        if (context.Request.Path.Value?.Contains("/health") == true)
        {
            await _next(context);
            return;
        }

        var userIdHeader = context.Request.Headers["X-User-Id"].FirstOrDefault();
        if (string.IsNullOrEmpty(userIdHeader) || !Guid.TryParse(userIdHeader, out var userId))
        {
            _logger.LogWarning("Request missing X-User-Id header from {RemoteIp}", context.Connection.RemoteIpAddress);
            context.Response.StatusCode = 401;
            await context.Response.WriteAsJsonAsync(new { message = "Unauthorized" });
            return;
        }

        context.Items["UserId"] = userId;
        context.Items["UserRole"] = context.Request.Headers["X-User-Role"].FirstOrDefault() ?? "user";
        await _next(context);
    }
}
