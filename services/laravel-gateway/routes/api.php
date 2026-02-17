<?php

use Illuminate\Support\Facades\Route;
use App\Http\Controllers\AuthController;
use App\Http\Controllers\GatewayController;
use App\Http\Middleware\JwtMiddleware;

Route::prefix('auth')->group(function () {
    Route::post('/register', [AuthController::class, 'register']);
    Route::post('/login',    [AuthController::class, 'login']);
    Route::post('/refresh',  [AuthController::class, 'refresh']);
    Route::post('/logout',   [AuthController::class, 'logout'])->middleware(JwtMiddleware::class);
    Route::get('/me',        [AuthController::class, 'me'])->middleware(JwtMiddleware::class);
});

Route::middleware(JwtMiddleware::class)->group(function () {

    Route::prefix('compile')->group(function () {
        Route::post('/',              [GatewayController::class, 'proxyCompile']);
        Route::get('/{jobId}/status', [GatewayController::class, 'proxyCompileStatus']);
        Route::post('/execute',       [GatewayController::class, 'proxyExecute']);
    });

    Route::prefix('projects')->group(function () {
        Route::get('/',       [GatewayController::class, 'proxyProjects']);
        Route::post('/',      [GatewayController::class, 'proxyProjects']);
        Route::get('/{id}',   [GatewayController::class, 'proxyProjects']);
        Route::put('/{id}',   [GatewayController::class, 'proxyProjects']);
        Route::delete('/{id}',[GatewayController::class, 'proxyProjects']);

        Route::get('/{projectId}/tasks',             [GatewayController::class, 'proxyTasks']);
        Route::post('/{projectId}/tasks',            [GatewayController::class, 'proxyTasks']);
        Route::get('/{projectId}/tasks/{taskId}',    [GatewayController::class, 'proxyTasks']);
        Route::put('/{projectId}/tasks/{taskId}',    [GatewayController::class, 'proxyTasks']);
        Route::delete('/{projectId}/tasks/{taskId}', [GatewayController::class, 'proxyTasks']);
    });

    Route::prefix('ml')->group(function () {
        Route::get('/jobs',            [GatewayController::class, 'proxyMlJobs']);
        Route::post('/jobs',           [GatewayController::class, 'proxyMlJobs']);
        Route::get('/jobs/{jobId}',    [GatewayController::class, 'proxyMlJobs']);
        Route::get('/models',          [GatewayController::class, 'proxyMlModels']);
        Route::post('/models',         [GatewayController::class, 'proxyMlModels']);
        Route::get('/models/{modelId}',[GatewayController::class, 'proxyMlModels']);
        Route::delete('/models/{modelId}',[GatewayController::class, 'proxyMlModels']);
    });

    Route::prefix('research')->group(function () {
        Route::get('/compile',           [GatewayController::class, 'proxyResearchCompile']);
        Route::post('/compile',          [GatewayController::class, 'proxyResearchCompile']);
        Route::get('/compile/{jobId}',   [GatewayController::class, 'proxyResearchCompile']);
    });

});
