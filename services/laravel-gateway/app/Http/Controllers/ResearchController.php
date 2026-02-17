<?php

namespace App\Http\Controllers;

use App\Models\ResearchJob;
use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Validator;
use Illuminate\Support\Str;

class ResearchController extends Controller
{
    private const SUPPORTED_LANGUAGES = ['zig', 'mojo', 'dafny', 'fstar'];

    public function compile(Request $request): JsonResponse
    {
        $validator = Validator::make($request->all(), [
            'project_id'     => 'required|uuid',
            'language'       => 'required|string|in:zig,mojo,dafny,fstar',
            'source_code'    => 'required|string|max:1048576',
            'compiler_flags' => 'nullable|string|max:512',
            'stdin_data'     => 'nullable|string|max:65536',
        ]);

        if ($validator->fails()) {
            return response()->json(['errors' => $validator->errors()], 422);
        }

        $user = $request->user();
        $jobId = (string) Str::uuid();

        $job = ResearchJob::create([
            'id'             => $jobId,
            'project_id'     => $request->project_id,
            'user_id'        => $user->id,
            'language'       => $request->language,
            'source_code'    => $request->source_code,
            'compiler_flags' => $request->compiler_flags,
            'status'         => 'running',
            'created_at'     => now(),
        ]);

        $runtimeUrl = env('RESEARCH_RUNTIME_URL', 'http://localhost:8004');

        try {
            $response = Http::withHeaders([
                'Content-Type' => 'application/json',
                'Accept'       => 'application/json',
            ])
            ->timeout(90)
            ->post("{$runtimeUrl}/compile", [
                'language'       => $request->language,
                'source_code'    => $request->source_code,
                'compiler_flags' => $request->compiler_flags,
                'stdin_data'     => $request->stdin_data,
            ]);

            $result = $response->json();

            $job->update([
                'status'              => $result['status'] ?? 'failed',
                'stdout'              => $result['stdout'] ?? null,
                'stderr'              => $result['stderr'] ?? null,
                'exit_code'           => $result['exit_code'] ?? null,
                'duration_ms'         => $result['duration_ms'] ?? null,
                'verification_passed' => $result['verification_passed'] ?? null,
                'completed_at'        => now(),
            ]);

            return response()->json([
                'job_id'              => $jobId,
                'language'            => $request->language,
                'status'              => $result['status'] ?? 'failed',
                'stdout'              => $result['stdout'] ?? '',
                'stderr'              => $result['stderr'] ?? '',
                'exit_code'           => $result['exit_code'] ?? null,
                'duration_ms'         => $result['duration_ms'] ?? null,
                'verification_passed' => $result['verification_passed'] ?? null,
            ]);
        } catch (\Illuminate\Http\Client\ConnectionException $e) {
            Log::error("research-runtime unreachable: {$e->getMessage()}");

            $job->update(['status' => 'failed', 'completed_at' => now()]);

            return response()->json(['message' => 'Research runtime service unreachable'], 503);
        } catch (\Exception $e) {
            Log::error("ResearchController error: {$e->getMessage()}");

            $job->update(['status' => 'failed', 'completed_at' => now()]);

            return response()->json(['message' => 'Internal error'], 500);
        }
    }

    public function index(Request $request): JsonResponse
    {
        $user = $request->user();
        $page = max(1, (int) $request->query('page', 1));
        $pageSize = min(50, max(1, (int) $request->query('page_size', 20)));

        $jobs = ResearchJob::where('user_id', $user->id)
            ->orderByDesc('created_at')
            ->skip(($page - 1) * $pageSize)
            ->take($pageSize)
            ->get(['id', 'language', 'status', 'exit_code', 'duration_ms', 'verification_passed', 'created_at', 'completed_at']);

        return response()->json($jobs);
    }

    public function show(Request $request, string $jobId): JsonResponse
    {
        $user = $request->user();

        $job = ResearchJob::where('id', $jobId)
            ->where('user_id', $user->id)
            ->first();

        if (!$job) {
            return response()->json(['message' => 'Research job not found'], 404);
        }

        return response()->json($job);
    }
}
