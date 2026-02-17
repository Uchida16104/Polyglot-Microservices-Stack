<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class ResearchJob extends Model
{
    protected $table = 'research_jobs';
    public $incrementing = false;
    protected $keyType = 'string';
    public $timestamps = false;

    protected $fillable = [
        'id', 'project_id', 'user_id', 'language', 'source_code',
        'compiler_flags', 'status', 'stdout', 'stderr', 'exit_code',
        'duration_ms', 'verification_passed', 'retry_count',
        'created_at', 'completed_at',
    ];

    protected $casts = [
        'verification_passed' => 'boolean',
        'created_at'          => 'datetime',
        'completed_at'        => 'datetime',
    ];

    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }
}
