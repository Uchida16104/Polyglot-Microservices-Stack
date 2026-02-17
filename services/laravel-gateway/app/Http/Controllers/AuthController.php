<?php

namespace App\Http\Controllers;

use App\Models\User;
use App\Models\Session;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\Hash;
use Illuminate\Support\Facades\Validator;
use Illuminate\Support\Str;
use Illuminate\Support\Facades\Cache;

class AuthController extends Controller
{
    public function register(Request $request): JsonResponse
    {
        $validator = Validator::make($request->all(), [
            'name'     => 'required|string|max:255',
            'email'    => 'required|string|email|max:255|unique:users',
            'password' => 'required|string|min:8|confirmed',
        ]);

        if ($validator->fails()) {
            return response()->json(['errors' => $validator->errors()], 422);
        }

        $user = User::create([
            'id'            => Str::uuid(),
            'name'          => $request->name,
            'email'         => $request->email,
            'password_hash' => Hash::make($request->password),
            'role'          => 'user',
            'status'        => 'active',
        ]);

        $tokens = $this->issueTokens($user, $request->ip(), $request->userAgent());

        return response()->json([
            'user'          => $user->toPublicArray(),
            'access_token'  => $tokens['access_token'],
            'refresh_token' => $tokens['refresh_token'],
            'token_type'    => 'Bearer',
        ], 201);
    }

    public function login(Request $request): JsonResponse
    {
        $validator = Validator::make($request->all(), [
            'email'    => 'required|email',
            'password' => 'required|string',
        ]);

        if ($validator->fails()) {
            return response()->json(['errors' => $validator->errors()], 422);
        }

        $user = User::where('email', $request->email)->first();

        if (!$user || !Hash::check($request->password, $user->password_hash)) {
            return response()->json(['message' => 'Invalid credentials'], 401);
        }

        if ($user->status !== 'active') {
            return response()->json(['message' => 'Account is not active'], 403);
        }

        $tokens = $this->issueTokens($user, $request->ip(), $request->userAgent());

        return response()->json([
            'user'          => $user->toPublicArray(),
            'access_token'  => $tokens['access_token'],
            'refresh_token' => $tokens['refresh_token'],
            'token_type'    => 'Bearer',
        ]);
    }

    public function refresh(Request $request): JsonResponse
    {
        $validator = Validator::make($request->all(), [
            'refresh_token' => 'required|string',
        ]);

        if ($validator->fails()) {
            return response()->json(['errors' => $validator->errors()], 422);
        }

        $tokenHash = hash('sha256', $request->refresh_token);
        $session = Session::where('token_hash', $tokenHash)
            ->where('expires_at', '>', now())
            ->first();

        if (!$session) {
            return response()->json(['message' => 'Invalid or expired refresh token'], 401);
        }

        $user = User::find($session->user_id);
        if (!$user || $user->status !== 'active') {
            return response()->json(['message' => 'User not found or inactive'], 401);
        }

        Cache::forget("session:{$tokenHash}");
        $session->delete();

        $tokens = $this->issueTokens($user, $request->ip(), $request->userAgent());

        return response()->json([
            'access_token'  => $tokens['access_token'],
            'refresh_token' => $tokens['refresh_token'],
            'token_type'    => 'Bearer',
        ]);
    }

    public function logout(Request $request): JsonResponse
    {
        $bearerToken = $request->bearerToken();
        if ($bearerToken) {
            $tokenHash = hash('sha256', $bearerToken);
            Cache::forget("session:{$tokenHash}");
            Session::where('token_hash', $tokenHash)->delete();
        }

        return response()->json(['message' => 'Successfully logged out']);
    }

    public function me(Request $request): JsonResponse
    {
        return response()->json(['user' => $request->user()->toPublicArray()]);
    }

    private function issueTokens(User $user, ?string $ip, ?string $userAgent): array
    {
        $accessToken  = Str::random(64);
        $refreshToken = Str::random(80);

        $session = Session::create([
            'id'         => Str::uuid(),
            'user_id'    => $user->id,
            'token_hash' => hash('sha256', $refreshToken),
            'ip_address' => $ip,
            'user_agent' => $userAgent,
            'expires_at' => now()->addDays(30),
        ]);

        Cache::put(
            "session:{$accessToken}",
            ['user_id' => $user->id, 'session_id' => $session->id],
            now()->addDay()
        );

        return [
            'access_token'  => $accessToken,
            'refresh_token' => $refreshToken,
        ];
    }
}
