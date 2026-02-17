using BusinessService.Models;
using BusinessService.Services;
using Microsoft.AspNetCore.Mvc;

namespace BusinessService.Controllers;

[ApiController]
[Route("projects/{projectId:guid}/tasks")]
public class TasksController : ControllerBase
{
    private readonly ITaskService _taskService;
    private readonly IAuditService _auditService;
    private readonly ILogger<TasksController> _logger;

    private static readonly HashSet<string> ValidStatuses = new()
    {
        "backlog", "todo", "in_progress", "blocked", "in_review", "done", "cancelled"
    };

    private static readonly Dictionary<string, HashSet<string>> AllowedTransitions = new()
    {
        ["backlog"]     = new() { "todo", "cancelled" },
        ["todo"]        = new() { "in_progress", "backlog" },
        ["in_progress"] = new() { "in_review", "blocked", "todo" },
        ["blocked"]     = new() { "in_progress", "cancelled" },
        ["in_review"]   = new() { "done", "in_progress", "blocked" },
        ["done"]        = new(),
        ["cancelled"]   = new(),
    };

    public TasksController(
        ITaskService taskService,
        IAuditService auditService,
        ILogger<TasksController> logger)
    {
        _taskService = taskService;
        _auditService = auditService;
        _logger = logger;
    }

    private Guid CurrentUserId =>
        Guid.Parse(HttpContext.Items["UserId"]?.ToString() ?? Guid.Empty.ToString());

    [HttpGet]
    public async Task<IActionResult> GetTasks(
        Guid projectId,
        [FromQuery] string? status,
        [FromQuery] int page = 1,
        [FromQuery] int pageSize = 50)
    {
        var tasks = await _taskService.GetProjectTasksAsync(projectId, CurrentUserId, status, page, pageSize);
        return Ok(tasks);
    }

    [HttpPost]
    public async Task<IActionResult> CreateTask(Guid projectId, [FromBody] CreateTaskRequest request)
    {
        var task = await _taskService.CreateTaskAsync(projectId, CurrentUserId, request);
        if (task == null) return BadRequest(new { message = "Project not found or access denied" });
        await _auditService.LogAsync(CurrentUserId, "csharp-business", "create", "task", task.Id.ToString());
        return CreatedAtAction(nameof(GetTask), new { projectId, taskId = task.Id }, task);
    }

    [HttpGet("{taskId:guid}")]
    public async Task<IActionResult> GetTask(Guid projectId, Guid taskId)
    {
        var task = await _taskService.GetTaskAsync(projectId, taskId, CurrentUserId);
        if (task == null) return NotFound(new { message = "Task not found" });
        return Ok(task);
    }

    [HttpPut("{taskId:guid}")]
    public async Task<IActionResult> UpdateTask(Guid projectId, Guid taskId, [FromBody] UpdateTaskRequest request)
    {
        if (request.Status != null)
        {
            var current = await _taskService.GetTaskStatusAsync(taskId);
            if (current != null && !AllowedTransitions.GetValueOrDefault(current, new())!.Contains(request.Status))
            {
                return UnprocessableEntity(new
                {
                    message = $"Invalid status transition from '{current}' to '{request.Status}'"
                });
            }
        }

        var task = await _taskService.UpdateTaskAsync(projectId, taskId, CurrentUserId, request);
        if (task == null) return NotFound(new { message = "Task not found" });
        await _auditService.LogAsync(CurrentUserId, "csharp-business", "update", "task", taskId.ToString());
        return Ok(task);
    }

    [HttpDelete("{taskId:guid}")]
    public async Task<IActionResult> DeleteTask(Guid projectId, Guid taskId)
    {
        var deleted = await _taskService.DeleteTaskAsync(projectId, taskId, CurrentUserId);
        if (!deleted) return NotFound(new { message = "Task not found" });
        await _auditService.LogAsync(CurrentUserId, "csharp-business", "delete", "task", taskId.ToString());
        return NoContent();
    }
}
