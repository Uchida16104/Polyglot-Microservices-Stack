using BusinessService.Data;
using BusinessService.Models;
using Microsoft.EntityFrameworkCore;

namespace BusinessService.Services;

public interface ITaskService
{
    Task<IEnumerable<TaskItem>> GetProjectTasksAsync(Guid projectId, Guid userId, string? status, int page, int pageSize);
    Task<TaskItem?> GetTaskAsync(Guid projectId, Guid taskId, Guid userId);
    Task<string?> GetTaskStatusAsync(Guid taskId);
    Task<TaskItem?> CreateTaskAsync(Guid projectId, Guid creatorId, CreateTaskRequest request);
    Task<TaskItem?> UpdateTaskAsync(Guid projectId, Guid taskId, Guid userId, UpdateTaskRequest request);
    Task<bool> DeleteTaskAsync(Guid projectId, Guid taskId, Guid userId);
}

public class TaskService : ITaskService
{
    private readonly AppDbContext _db;

    public TaskService(AppDbContext db) => _db = db;

    public async Task<IEnumerable<TaskItem>> GetProjectTasksAsync(
        Guid projectId, Guid userId, string? status, int page, int pageSize)
    {
        var query = _db.Tasks
            .Where(t => t.ProjectId == projectId)
            .Where(t => t.Project!.OwnerId == userId ||
                        t.Project.Members.Any(m => m.UserId == userId));

        if (!string.IsNullOrEmpty(status))
            query = query.Where(t => t.Status == status);

        return await query
            .OrderBy(t => t.OrderIndex)
            .ThenByDescending(t => t.CreatedAt)
            .Skip((page - 1) * pageSize)
            .Take(pageSize)
            .ToListAsync();
    }

    public async Task<TaskItem?> GetTaskAsync(Guid projectId, Guid taskId, Guid userId)
    {
        return await _db.Tasks
            .Where(t => t.Id == taskId && t.ProjectId == projectId)
            .Where(t => t.Project!.OwnerId == userId ||
                        t.Project.Members.Any(m => m.UserId == userId))
            .FirstOrDefaultAsync();
    }

    public async Task<string?> GetTaskStatusAsync(Guid taskId)
    {
        var task = await _db.Tasks.AsNoTracking().Select(t => new { t.Id, t.Status }).FirstOrDefaultAsync(t => t.Id == taskId);
        return task?.Status;
    }

    public async Task<TaskItem?> CreateTaskAsync(Guid projectId, Guid creatorId, CreateTaskRequest request)
    {
        var hasAccess = await _db.Projects.AnyAsync(p =>
            p.Id == projectId &&
            (p.OwnerId == creatorId || p.Members.Any(m => m.UserId == creatorId && m.Role != "viewer")));

        if (!hasAccess) return null;

        var maxOrder = await _db.Tasks
            .Where(t => t.ProjectId == projectId)
            .MaxAsync(t => (int?)t.OrderIndex) ?? 0;

        var task = new TaskItem
        {
            Id = Guid.NewGuid(),
            ProjectId = projectId,
            CreatorId = creatorId,
            AssigneeId = request.AssigneeId,
            Title = request.Title,
            Description = request.Description,
            Priority = request.Priority,
            Status = "backlog",
            OrderIndex = maxOrder + 1,
            DueAt = request.DueAt,
            CreatedAt = DateTime.UtcNow,
            UpdatedAt = DateTime.UtcNow,
        };

        _db.Tasks.Add(task);
        await _db.SaveChangesAsync();
        return task;
    }

    public async Task<TaskItem?> UpdateTaskAsync(
        Guid projectId, Guid taskId, Guid userId, UpdateTaskRequest request)
    {
        var task = await _db.Tasks
            .Where(t => t.Id == taskId && t.ProjectId == projectId)
            .Where(t => t.Project!.OwnerId == userId ||
                        t.Project.Members.Any(m => m.UserId == userId && m.Role != "viewer"))
            .FirstOrDefaultAsync();

        if (task == null) return null;

        if (request.Title != null) task.Title = request.Title;
        if (request.Description != null) task.Description = request.Description;
        if (request.Priority != null) task.Priority = request.Priority;
        if (request.Status != null) task.Status = request.Status;
        if (request.AssigneeId.HasValue) task.AssigneeId = request.AssigneeId.Value;
        if (request.OrderIndex.HasValue) task.OrderIndex = request.OrderIndex.Value;
        if (request.DueAt.HasValue) task.DueAt = request.DueAt.Value;
        task.UpdatedAt = DateTime.UtcNow;

        await _db.SaveChangesAsync();
        return task;
    }

    public async Task<bool> DeleteTaskAsync(Guid projectId, Guid taskId, Guid userId)
    {
        var task = await _db.Tasks
            .Where(t => t.Id == taskId && t.ProjectId == projectId)
            .Where(t => t.Project!.OwnerId == userId ||
                        t.Project.Members.Any(m => m.UserId == userId && (m.Role == "owner" || m.Role == "admin")))
            .FirstOrDefaultAsync();

        if (task == null) return false;

        _db.Tasks.Remove(task);
        await _db.SaveChangesAsync();
        return true;
    }
}
