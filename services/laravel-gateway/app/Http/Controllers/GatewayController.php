<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Log;

class GatewayController extends Controller
{
    private array $serviceMap = [
        'rust-core'          => 'RUST_CORE_URL',
        'csharp-business'    => 'CSHARP_BUSINESS_URL',
        'python-ml'          => 'PYTHON_ML_URL',
        'research-runtimes'  => 'RESEARCH_RUNTIMES_URL',
    ];

    public function proxyCompile(Request $request): JsonResponse
    {
        return $this->proxy($request, 'rust-core', '/compile');
    }

    public function proxyCompileStatus(Request $request, string $jobId): JsonResponse
    {
        return $this->proxy($request, 'rust-core', "/compile/{$jobId}/status");
    }

    public function proxyExecute(Request $request): JsonResponse
    {
        return $this->proxy($request, 'rust-core', '/execute');
    }

    public function proxyProjects(Request $request, ?string $id = null): JsonResponse
    {
        $path = $id ? "/projects/{$id}" : '/projects';
        return $this->proxy($request, 'csharp-business', $path);
    }

    public function proxyTasks(Request $request, string $projectId, ?string $taskId = null): JsonResponse
    {
        $path = $taskId
            ? "/projects/{$projectId}/tasks/{$taskId}"
            : "/projects/{$projectId}/tasks";
        return $this->proxy($request, 'csharp-business', $path);
    }

    public function proxyMlJobs(Request $request, ?string $jobId = null): JsonResponse
    {
        $path = $jobId ? "/jobs/{$jobId}" : '/jobs';
        return $this->proxy($request, 'python-ml', $path);
    }

    public function proxyMlModels(Request $request, ?string $modelId = null): JsonResponse
    {
        $path = $modelId ? "/models/{$modelId}" : '/models';
        return $this->proxy($request, 'python-ml', $path);
    }

    public function proxyResearchCompile(Request $request, ?string $jobId = null): JsonResponse
    {
        $path = $jobId ? "/compile/{$jobId}" : '/compile';
        return $this->proxy($request, 'research-runtimes', $path);
    }

    private function proxy(Request $request, string $service, string $path): JsonResponse
    {
        $baseUrl = env($this->serviceMap[$service]);
        if (!$baseUrl) {
            Log::error("Service URL not configured: {$service}");
            return response()->json(['message' => 'Service unavailable'], 503);
        }

        $user = $request->user();
        $headers = [
            'X-User-Id'    => $user?->id,
            'X-User-Role'  => $user?->role,
            'X-Request-Id' => $request->header('X-Request-Id', (string) \Illuminate\Support\Str::uuid()),
            'Content-Type' => 'application/json',
            'Accept'       => 'application/json',
        ];

        try {
            $response = Http::withHeaders($headers)
                ->timeout(30)
                ->send(
                    $request->method(),
                    rtrim($baseUrl, '/') . $path,
                    [
                        'json'  => $request->isMethod('GET') ? null : $request->all(),
                        'query' => $request->isMethod('GET') ? $request->all() : [],
                    ]
                );

            return response()->json($response->json(), $response->status());
        } catch (\Illuminate\Http\Client\ConnectionException $e) {
            Log::error("Gateway proxy error [{$service}]: " . $e->getMessage());
            return response()->json(['message' => 'Upstream service unreachable'], 503);
        } catch (\Exception $e) {
            Log::error("Gateway unexpected error [{$service}]: " . $e->getMessage());
            return response()->json(['message' => 'Internal gateway error'], 500);
        }
    }
}
