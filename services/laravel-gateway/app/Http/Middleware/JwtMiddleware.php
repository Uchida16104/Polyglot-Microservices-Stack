<?php

namespace App\Http\Middleware;

use Closure;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Cache;
use App\Models\User;
use Symfony\Component\HttpFoundation\Response;

class JwtMiddleware
{
    public function handle(Request $request, Closure $next): Response
    {
        $token = $request->bearerToken();

        if (!$token) {
            return response()->json(['message' => 'Unauthenticated'], 401);
        }

        $sessionData = Cache::get("session:{$token}");

        if (!$sessionData) {
            return response()->json(['message' => 'Token expired or invalid'], 401);
        }

        $user = Cache::remember(
            "user:{$sessionData['user_id']}",
            300,
            fn () => User::find($sessionData['user_id'])
        );

        if (!$user || $user->status !== 'active') {
            Cache::forget("session:{$token}");
            return response()->json(['message' => 'User not found or inactive'], 401);
        }

        $request->setUserResolver(fn () => $user);

        return $next($request);
    }
}
