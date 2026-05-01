"""
Brother IDE AI - FastAPI Backend
Local AI-powered API for the Brother IDE AI editor.
Uses Ollama with DeepSeek models for code reasoning, translation, and security analysis.
"""

import os
import subprocess
import logging
from typing import Optional

from fastapi import FastAPI, HTTPException, Query
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
import httpx

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger("brother-api")

OLLAMA_URL = os.getenv("OLLAMA_URL", "http://localhost:11434")
MODEL = os.getenv("BROTHER_MODEL", "deepseek-r1:7b")

app = FastAPI(
    title="Brother IDE AI API",
    description="Local AI-powered API for Brother IDE AI editor",
    version="1.0.0",
)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


class NL2CmdRequest(BaseModel):
    input: str


class NL2CmdResponse(BaseModel):
    input: str
    command: str
    explanation: str


class PackageInstallRequest(BaseModel):
    package_name: str
    package_manager: Optional[str] = None


class PackageInstallResponse(BaseModel):
    package_name: str
    package_manager: str
    success: bool
    output: str


class SecurityScanRequest(BaseModel):
    input: str


class SecurityScanResponse(BaseModel):
    input_preview: str
    issues: list[dict]
    summary: str


async def query_ollama(prompt: str, system_prompt: str = "") -> str:
    """Send a prompt to the local Ollama instance and return the response."""
    try:
        async with httpx.AsyncClient(timeout=120.0) as client:
            payload = {
                "model": MODEL,
                "prompt": prompt,
                "stream": False,
            }
            if system_prompt:
                payload["system"] = system_prompt

            response = await client.post(
                f"{OLLAMA_URL}/api/generate",
                json=payload,
            )
            response.raise_for_status()
            data = response.json()
            return data.get("response", "No response from model.")
    except httpx.ConnectError:
        raise HTTPException(
            status_code=503,
            detail=(
                "Ollama is not running. Please start it with: ollama serve"
            ),
        )
    except httpx.TimeoutException:
        raise HTTPException(
            status_code=504,
            detail="Request to Ollama timed out. The model may be loading.",
        )
    except httpx.HTTPStatusError as error:
        raise HTTPException(
            status_code=502,
            detail=f"Ollama returned an error: {error.response.status_code}",
        )


@app.get("/health")
async def health_check():
    """Health check endpoint."""
    ollama_status = "unknown"
    try:
        async with httpx.AsyncClient(timeout=5.0) as client:
            response = await client.get(f"{OLLAMA_URL}/api/tags")
            if response.status_code == 200:
                ollama_status = "connected"
            else:
                ollama_status = "error"
    except (httpx.ConnectError, httpx.TimeoutException):
        ollama_status = "disconnected"

    return {
        "status": "ok",
        "service": "Brother IDE AI API",
        "model": MODEL,
        "ollama": ollama_status,
    }


@app.get("/v1/omni/reason")
async def omni_reason(request: str = Query(..., description="The question or code to reason about")):
    """General-purpose AI reasoning endpoint.
    Accepts a query and returns the AI model's response.
    """
    system_prompt = (
        "You are Brother AI, a helpful coding assistant integrated into "
        "Brother IDE AI. Provide clear, concise, and accurate answers about "
        "code, programming, and software engineering. When showing code, "
        "include language identifiers in code blocks."
    )
    response = await query_ollama(request, system_prompt)
    return {"request": request, "response": response, "model": MODEL}


@app.post("/v1/translate/nl2cmd")
async def translate_nl_to_cmd(body: NL2CmdRequest):
    """Translate natural language instructions into shell commands.
    Uses the AI model to convert human-readable instructions into executable
    shell commands, similar to developergpt.
    """
    system_prompt = (
        "You are a command-line expert. Convert the user's natural language "
        "instruction into the appropriate shell command for a Linux system. "
        "Respond ONLY with a JSON object containing two fields:\n"
        '- "command": the shell command to execute\n'
        '- "explanation": a brief explanation of what the command does\n'
        "Do not include markdown formatting or code blocks in your response."
    )
    raw_response = await query_ollama(body.input, system_prompt)

    try:
        import json
        parsed = json.loads(raw_response)
        command = parsed.get("command", raw_response)
        explanation = parsed.get("explanation", "")
    except (json.JSONDecodeError, ValueError):
        command = raw_response.strip()
        explanation = "Raw model output (could not parse structured response)"

    return NL2CmdResponse(
        input=body.input,
        command=command,
        explanation=explanation,
    )


@app.post("/v1/package/install")
async def package_install(body: PackageInstallRequest):
    """Install a system or Python package using the appropriate package manager.
    Determines the best package manager to use and installs the requested package.
    All installations run through a controlled subprocess.
    """
    package_manager = body.package_manager

    if not package_manager:
        for manager in ["apt-get", "dnf", "pacman", "pip3"]:
            try:
                subprocess.run(
                    ["which", manager],
                    capture_output=True,
                    check=True,
                )
                package_manager = manager
                break
            except subprocess.CalledProcessError:
                continue

    if not package_manager:
        raise HTTPException(
            status_code=400,
            detail="No supported package manager found on this system.",
        )

    command_map = {
        "apt-get": ["sudo", "apt-get", "install", "-y", body.package_name],
        "dnf": ["sudo", "dnf", "install", "-y", body.package_name],
        "pacman": ["sudo", "pacman", "-S", "--noconfirm", body.package_name],
        "pip3": ["pip3", "install", body.package_name],
        "pip": ["pip", "install", body.package_name],
    }

    install_command = command_map.get(package_manager)
    if not install_command:
        raise HTTPException(
            status_code=400,
            detail=f"Unsupported package manager: {package_manager}",
        )

    try:
        result = subprocess.run(
            install_command,
            capture_output=True,
            text=True,
            timeout=300,
        )
        return PackageInstallResponse(
            package_name=body.package_name,
            package_manager=package_manager,
            success=result.returncode == 0,
            output=result.stdout if result.returncode == 0 else result.stderr,
        )
    except subprocess.TimeoutExpired:
        raise HTTPException(
            status_code=504,
            detail=f"Installation of {body.package_name} timed out after 300 seconds.",
        )


@app.post("/v1/security/scan")
async def security_scan(body: SecurityScanRequest):
    """Scan code for security vulnerabilities using AI analysis."""
    system_prompt = (
        "You are a security expert analyzing code for vulnerabilities. "
        "Analyze the provided code and identify security issues. "
        "Respond with a JSON object containing:\n"
        '- "issues": an array of objects, each with "severity" (critical/high/medium/low), '
        '"type" (e.g. "SQL Injection", "XSS"), "line_hint" (approximate line), '
        'and "description"\n'
        '- "summary": a brief overall assessment\n'
        "If no issues are found, return an empty issues array with an appropriate summary."
    )
    raw_response = await query_ollama(
        f"Analyze this code for security vulnerabilities:\n\n{body.input}",
        system_prompt,
    )

    try:
        import json
        parsed = json.loads(raw_response)
        issues = parsed.get("issues", [])
        summary = parsed.get("summary", raw_response)
    except (json.JSONDecodeError, ValueError):
        issues = []
        summary = raw_response

    input_preview = body.input[:200] + "..." if len(body.input) > 200 else body.input
    return SecurityScanResponse(
        input_preview=input_preview,
        issues=issues,
        summary=summary,
    )


@app.get("/v1/models")
async def list_models():
    """List available Ollama models."""
    try:
        async with httpx.AsyncClient(timeout=10.0) as client:
            response = await client.get(f"{OLLAMA_URL}/api/tags")
            response.raise_for_status()
            return response.json()
    except httpx.ConnectError:
        raise HTTPException(
            status_code=503,
            detail="Ollama is not running.",
        )


if __name__ == "__main__":
    import uvicorn
    port = int(os.getenv("BROTHER_API_PORT", "8001"))
    logger.info("Starting Brother IDE AI API on port %d", port)
    uvicorn.run(app, host="0.0.0.0", port=port)
