using BusinessService.Models;
using Microsoft.EntityFrameworkCore;

namespace BusinessService.Data;

public class AppDbContext : DbContext
{
    public AppDbContext(DbContextOptions<AppDbContext> options) : base(options) { }

    public DbSet<Project> Projects => Set<Project>();
    public DbSet<ProjectMember> ProjectMembers => Set<ProjectMember>();
    public DbSet<TaskItem> Tasks => Set<TaskItem>();
    public DbSet<AuditLog> AuditLogs => Set<AuditLog>();

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        modelBuilder.Entity<Project>(e =>
        {
            e.ToTable("projects");
            e.HasKey(p => p.Id);
            e.Property(p => p.Id).HasColumnName("id").HasDefaultValueSql("gen_random_uuid()");
            e.Property(p => p.OwnerId).HasColumnName("owner_id").IsRequired();
            e.Property(p => p.Name).HasColumnName("name").HasMaxLength(255).IsRequired();
            e.Property(p => p.Description).HasColumnName("description");
            e.Property(p => p.Visibility).HasColumnName("visibility").HasMaxLength(50).HasDefaultValue("private");
            e.Property(p => p.Status).HasColumnName("status").HasMaxLength(50).HasDefaultValue("active");
            e.Property(p => p.CreatedAt).HasColumnName("created_at").HasDefaultValueSql("now()");
            e.Property(p => p.UpdatedAt).HasColumnName("updated_at").HasDefaultValueSql("now()");
            e.HasMany(p => p.Members).WithOne(m => m.Project).HasForeignKey(m => m.ProjectId);
            e.HasMany(p => p.Tasks).WithOne(t => t.Project).HasForeignKey(t => t.ProjectId);
        });

        modelBuilder.Entity<ProjectMember>(e =>
        {
            e.ToTable("project_members");
            e.HasKey(pm => pm.Id);
            e.Property(pm => pm.Id).HasColumnName("id").HasDefaultValueSql("gen_random_uuid()");
            e.Property(pm => pm.ProjectId).HasColumnName("project_id").IsRequired();
            e.Property(pm => pm.UserId).HasColumnName("user_id").IsRequired();
            e.Property(pm => pm.Role).HasColumnName("role").HasMaxLength(50).HasDefaultValue("member");
            e.Property(pm => pm.JoinedAt).HasColumnName("joined_at").HasDefaultValueSql("now()");
            e.HasIndex(pm => new { pm.ProjectId, pm.UserId }).IsUnique();
        });

        modelBuilder.Entity<TaskItem>(e =>
        {
            e.ToTable("tasks");
            e.HasKey(t => t.Id);
            e.Property(t => t.Id).HasColumnName("id").HasDefaultValueSql("gen_random_uuid()");
            e.Property(t => t.ProjectId).HasColumnName("project_id").IsRequired();
            e.Property(t => t.AssigneeId).HasColumnName("assignee_id");
            e.Property(t => t.CreatorId).HasColumnName("creator_id").IsRequired();
            e.Property(t => t.Title).HasColumnName("title").HasMaxLength(500).IsRequired();
            e.Property(t => t.Description).HasColumnName("description");
            e.Property(t => t.Priority).HasColumnName("priority").HasMaxLength(50).HasDefaultValue("medium");
            e.Property(t => t.Status).HasColumnName("status").HasMaxLength(50).HasDefaultValue("backlog");
            e.Property(t => t.OrderIndex).HasColumnName("order_index").HasDefaultValue(0);
            e.Property(t => t.DueAt).HasColumnName("due_at");
            e.Property(t => t.CreatedAt).HasColumnName("created_at").HasDefaultValueSql("now()");
            e.Property(t => t.UpdatedAt).HasColumnName("updated_at").HasDefaultValueSql("now()");
        });

        modelBuilder.Entity<AuditLog>(e =>
        {
            e.ToTable("audit_logs");
            e.HasKey(a => a.Id);
            e.Property(a => a.Id).HasColumnName("id").HasDefaultValueSql("gen_random_uuid()");
            e.Property(a => a.UserId).HasColumnName("user_id");
            e.Property(a => a.Service).HasColumnName("service").IsRequired();
            e.Property(a => a.Action).HasColumnName("action").IsRequired();
            e.Property(a => a.ResourceType).HasColumnName("resource_type").IsRequired();
            e.Property(a => a.ResourceId).HasColumnName("resource_id");
            e.Property(a => a.Metadata).HasColumnName("metadata").HasColumnType("jsonb");
            e.Property(a => a.IpAddress).HasColumnName("ip_address");
            e.Property(a => a.CreatedAt).HasColumnName("created_at").HasDefaultValueSql("now()");
        });
    }
}
