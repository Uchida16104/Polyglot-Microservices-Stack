using System.ComponentModel.DataAnnotations;

namespace BusinessService.Models;

public class Project
{
    public Guid Id { get; set; }
    public Guid OwnerId { get; set; }
    [Required, MaxLength(255)]
    public string Name { get; set; } = string.Empty;
    public string? Description { get; set; }
    public string Visibility { get; set; } = "private";
    public string Status { get; set; } = "active";
    public DateTime CreatedAt { get; set; }
    public DateTime UpdatedAt { get; set; }
    public ICollection<ProjectMember> Members { get; set; } = new List<ProjectMember>();
    public ICollection<TaskItem> Tasks { get; set; } = new List<TaskItem>();
}

public class ProjectMember
{
    public Guid Id { get; set; }
    public Guid ProjectId { get; set; }
    public Guid UserId { get; set; }
    public string Role { get; set; } = "member";
    public DateTime JoinedAt { get; set; }
    public Project? Project { get; set; }
}

public class TaskItem
{
    public Guid Id { get; set; }
    public Guid ProjectId { get; set; }
    public Guid? AssigneeId { get; set; }
    public Guid CreatorId { get; set; }
    [Required, MaxLength(500)]
    public string Title { get; set; } = string.Empty;
    public string? Description { get; set; }
    public string Priority { get; set; } = "medium";
    public string Status { get; set; } = "backlog";
    public int OrderIndex { get; set; }
    public DateTime? DueAt { get; set; }
    public DateTime CreatedAt { get; set; }
    public DateTime UpdatedAt { get; set; }
    public Project? Project { get; set; }
}

public class AuditLog
{
    public Guid Id { get; set; }
    public Guid? UserId { get; set; }
    public string Service { get; set; } = string.Empty;
    public string Action { get; set; } = string.Empty;
    public string ResourceType { get; set; } = string.Empty;
    public string? ResourceId { get; set; }
    public string? Metadata { get; set; }
    public string? IpAddress { get; set; }
    public DateTime CreatedAt { get; set; }
}

public record CreateProjectRequest(
    [Required, MaxLength(255)] string Name,
    string? Description,
    string Visibility = "private"
);

public record UpdateProjectRequest(
    string? Name,
    string? Description,
    string? Visibility,
    string? Status
);

public record CreateTaskRequest(
    [Required, MaxLength(500)] string Title,
    string? Description,
    string Priority = "medium",
    Guid? AssigneeId = null,
    DateTime? DueAt = null
);

public record UpdateTaskRequest(
    string? Title,
    string? Description,
    string? Priority,
    string? Status,
    Guid? AssigneeId,
    int? OrderIndex,
    DateTime? DueAt
);
