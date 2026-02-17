using BusinessService.Models;
using BusinessService.Services;
using Microsoft.AspNetCore.Mvc;

namespace BusinessService.Controllers;

[ApiController]
[Route("projects")]
public class ProjectsController : ControllerBase
{
    private readonly IProjectService _projectService;
    private readonly IAuditService _auditService;
    private readonly ILogger<ProjectsController> _logger;

    public ProjectsController(
        IProjectService projectService,
        IAuditService auditService,
        ILogger<ProjectsController> logger)
    {
        _projectService = projectService;
        _auditService = auditService;
        _logger = logger;
    }

    private Guid CurrentUserId =>
        Guid.Parse(HttpContext.Items["UserId"]?.ToString() ?? Guid.Empty.ToString());

    [HttpGet]
    public async Task<IActionResult> GetProjects(
        [FromQuery] int page = 1,
        [FromQuery] int pageSize = 20)
    {
        var projects = await _projectService.GetUserProjectsAsync(CurrentUserId, page, pageSize);
        return Ok(projects);
    }

    [HttpPost]
    public async Task<IActionResult> CreateProject([FromBody] CreateProjectRequest request)
    {
        var project = await _projectService.CreateProjectAsync(CurrentUserId, request);
        await _auditService.LogAsync(CurrentUserId, "csharp-business", "create", "project", project.Id.ToString(), HttpContext.Connection.RemoteIpAddress?.ToString());
        return CreatedAtAction(nameof(GetProject), new { id = project.Id }, project);
    }

    [HttpGet("{id:guid}")]
    public async Task<IActionResult> GetProject(Guid id)
    {
        var project = await _projectService.GetProjectAsync(id, CurrentUserId);
        if (project == null) return NotFound(new { message = "Project not found" });
        return Ok(project);
    }

    [HttpPut("{id:guid}")]
    public async Task<IActionResult> UpdateProject(Guid id, [FromBody] UpdateProjectRequest request)
    {
        var project = await _projectService.UpdateProjectAsync(id, CurrentUserId, request);
        if (project == null) return NotFound(new { message = "Project not found" });
        await _auditService.LogAsync(CurrentUserId, "csharp-business", "update", "project", id.ToString(), HttpContext.Connection.RemoteIpAddress?.ToString());
        return Ok(project);
    }

    [HttpDelete("{id:guid}")]
    public async Task<IActionResult> DeleteProject(Guid id)
    {
        var deleted = await _projectService.DeleteProjectAsync(id, CurrentUserId);
        if (!deleted) return NotFound(new { message = "Project not found" });
        await _auditService.LogAsync(CurrentUserId, "csharp-business", "delete", "project", id.ToString(), HttpContext.Connection.RemoteIpAddress?.ToString());
        return NoContent();
    }

    [HttpGet("{id:guid}/members")]
    public async Task<IActionResult> GetMembers(Guid id)
    {
        var members = await _projectService.GetMembersAsync(id, CurrentUserId);
        if (members == null) return NotFound();
        return Ok(members);
    }

    [HttpPost("{id:guid}/members")]
    public async Task<IActionResult> AddMember(Guid id, [FromBody] AddMemberRequest request)
    {
        var member = await _projectService.AddMemberAsync(id, CurrentUserId, request.UserId, request.Role ?? "member");
        if (member == null) return BadRequest(new { message = "Could not add member" });
        return Ok(member);
    }
}

public record AddMemberRequest(Guid UserId, string? Role);
