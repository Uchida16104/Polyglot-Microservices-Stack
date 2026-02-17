<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;

class User extends Model
{
    protected $table = 'users';
    public $incrementing = false;
    protected $keyType = 'string';

    protected $fillable = [
        'id', 'email', 'password_hash', 'name', 'role', 'status', 'email_verified_at',
    ];

    protected $hidden = ['password_hash'];

    protected $casts = [
        'email_verified_at' => 'datetime',
        'created_at'        => 'datetime',
        'updated_at'        => 'datetime',
    ];

    public function sessions(): HasMany
    {
        return $this->hasMany(Session::class);
    }

    public function toPublicArray(): array
    {
        return [
            'id'                => $this->id,
            'name'              => $this->name,
            'email'             => $this->email,
            'role'              => $this->role,
            'status'            => $this->status,
            'email_verified_at' => $this->email_verified_at,
            'created_at'        => $this->created_at,
        ];
    }
}
