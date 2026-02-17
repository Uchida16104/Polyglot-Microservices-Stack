using BusinessService.Data;
using BusinessService.Models;

namespace BusinessService.Services;

public interface IAuditService
{
    Task LogAsync(Guid? userId, string service, string action, string resourceType, string? resourceId = null, string? ipAddress = null, string? metadata = null);
}

public class AuditService : IAuditService
{
    private readonly AppDbContext _db;

    public AuditService(AppDbContext db) => _db = db;

    public async Task LogAsync(
        Guid? userId,
        string service,
        string action,
        string resourceType,
        string? resourceId = null,
        string? ipAddress = null,
        string? metadata = null)
    {
        _db.AuditLogs.Add(new AuditLog
        {
            Id = Guid.NewGuid(),
            UserId = userId,
            Service = service,
            Action = action,
            ResourceType = resourceType,
            ResourceId = resourceId,
            IpAddress = ipAddress,
            Metadata = metadata,
            CreatedAt = DateTime.UtcNow,
        });

        await _db.SaveChangesAsync();
    }
}
