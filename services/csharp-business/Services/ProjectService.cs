using BusinessService.Data;
using BusinessService.Models;
using Microsoft.EntityFrameworkCore;

namespace BusinessService.Services;

public interface IProjectService
{
    Task<IEnumerable<Project>> GetUserProjectsAsync(Guid userId, int page, int pageSize);
    Task<Project?> GetProjectAsync(Guid projectId, Guid userId);
    Task<Project> CreateProjectAsync(Guid ownerId, CreateProjectRequest request);
    Task<Project?> UpdateProjectAsync(Guid projectId, Guid userId, UpdateProjectRequest request);
    Task<bool> DeleteProjectAsync(Guid projectId, Guid userId);
    Task<IEnumerable<ProjectMember>?> GetMembersAsync(Guid projectId, Guid userId);
    Task<ProjectMember?> AddMemberAsync(Guid projectId, Guid requesterId, Guid newUserId, string role);
}

public class ProjectService : IProjectService
{
    private readonly AppDbContext _db;

    public ProjectService(AppDbContext db) => _db = db;

    public async Task<IEnumerable<Project>> GetUserProjectsAsync(Guid userId, int page, int pageSize)
    {
        return await _db.Projects
            .Where(p => p.OwnerId == userId ||
                        p.Members.Any(m => m.UserId == userId))
            .Where(p => p.Status != "deleted")
            .OrderByDescending(p => p.UpdatedAt)
            .Skip((page - 1) * pageSize)
            .Take(pageSize)
            .Include(p => p.Members)
            .ToListAsync();
    }

    public async Task<Project?> GetProjectAsync(Guid projectId, Guid userId)
    {
        return await _db.Projects
            .Where(p => p.Id == projectId && p.Status != "deleted")
            .Where(p => p.OwnerId == userId || p.Members.Any(m => m.UserId == userId) || p.Visibility == "public")
            .Include(p => p.Members)
            .FirstOrDefaultAsync();
    }

    public async Task<Project> CreateProjectAsync(Guid ownerId, CreateProjectRequest request)
    {
        var project = new Project
        {
            Id = Guid.NewGuid(),
            OwnerId = ownerId,
            Name = request.Name,
            Description = request.Description,
            Visibility = request.Visibility,
            Status = "active",
            CreatedAt = DateTime.UtcNow,
            UpdatedAt = DateTime.UtcNow,
        };

        _db.Projects.Add(project);

        _db.ProjectMembers.Add(new ProjectMember
        {
            Id = Guid.NewGuid(),
            ProjectId = project.Id,
            UserId = ownerId,
            Role = "owner",
            JoinedAt = DateTime.UtcNow,
        });

        await _db.SaveChangesAsync();
        return project;
    }

    public async Task<Project?> UpdateProjectAsync(Guid projectId, Guid userId, UpdateProjectRequest request)
    {
        var project = await _db.Projects
            .FirstOrDefaultAsync(p => p.Id == projectId &&
                                      (p.OwnerId == userId || p.Members.Any(m => m.UserId == userId && m.Role != "viewer")));

        if (project == null) return null;

        if (request.Name != null) project.Name = request.Name;
        if (request.Description != null) project.Description = request.Description;
        if (request.Visibility != null) project.Visibility = request.Visibility;
        if (request.Status != null) project.Status = request.Status;
        project.UpdatedAt = DateTime.UtcNow;

        await _db.SaveChangesAsync();
        return project;
    }

    public async Task<bool> DeleteProjectAsync(Guid projectId, Guid userId)
    {
        var project = await _db.Projects
            .FirstOrDefaultAsync(p => p.Id == projectId && p.OwnerId == userId);

        if (project == null) return false;

        project.Status = "deleted";
        project.UpdatedAt = DateTime.UtcNow;
        await _db.SaveChangesAsync();
        return true;
    }

    public async Task<IEnumerable<ProjectMember>?> GetMembersAsync(Guid projectId, Guid userId)
    {
        var hasAccess = await _db.Projects.AnyAsync(p =>
            p.Id == projectId &&
            (p.OwnerId == userId || p.Members.Any(m => m.UserId == userId)));

        if (!hasAccess) return null;

        return await _db.ProjectMembers
            .Where(m => m.ProjectId == projectId)
            .ToListAsync();
    }

    public async Task<ProjectMember?> AddMemberAsync(Guid projectId, Guid requesterId, Guid newUserId, string role)
    {
        var isOwnerOrAdmin = await _db.ProjectMembers.AnyAsync(m =>
            m.ProjectId == projectId && m.UserId == requesterId &&
            (m.Role == "owner" || m.Role == "admin"));

        if (!isOwnerOrAdmin) return null;

        var existing = await _db.ProjectMembers
            .FirstOrDefaultAsync(m => m.ProjectId == projectId && m.UserId == newUserId);

        if (existing != null)
        {
            existing.Role = role;
            await _db.SaveChangesAsync();
            return existing;
        }

        var member = new ProjectMember
        {
            Id = Guid.NewGuid(),
            ProjectId = projectId,
            UserId = newUserId,
            Role = role,
            JoinedAt = DateTime.UtcNow,
        };

        _db.ProjectMembers.Add(member);
        await _db.SaveChangesAsync();
        return member;
    }
}
