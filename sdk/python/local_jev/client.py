"""
local-jev Python Client SDK: 100% Offline TypeSafe Jev System One Decisions.
"""

import urllib.request
import json
from typing import List, Dict, Any, Optional

class JevClient:
    """Client for querying local-jev decision engine over HTTP."""

    def __init__(self, base_url: str = "http://localhost:8080/v1"):
        self.base_url = base_url.rstrip("/")

    def _post(self, path: str, payload: Dict[str, Any]) -> Dict[str, Any]:
        url = f"{self.base_url}{path}"
        data = json.dumps(payload).encode("utf-8")
        req = urllib.request.Request(
            url,
            data=data,
            headers={"Content-Type": "application/json"}
        )
        with urllib.request.urlopen(req) as resp:
            return json.loads(resp.read().decode("utf-8"))

    def choice(
        self,
        state: str,
        question: str,
        options: List[str],
        temperature: float = 0.1
    ) -> Dict[str, Any]:
        """Single-pass constrained option selection."""
        return self._post("/systemone/choice", {
            "state": state,
            "question": question,
            "options": options,
            "temperature": temperature
        })

    def score(self, text: str, criteria: str) -> float:
        """Semantic relevance score (0.0 to 1.0)."""
        res = self._post("/systemone/score", {
            "text": text,
            "criteria": criteria
        })
        return res.get("score", 0.0)

    def route(self, prompt: str, routes: List[str]) -> str:
        """Fast intent route classification."""
        res = self._post("/systemone/route", {
            "prompt": prompt,
            "routes": routes
        })
        return res.get("route", "")

    def noul(self, state: str, candidates: List[str]) -> Dict[str, Any]:
        """Classify step into ACTION, OBSERVATION, or THOUGHT."""
        return self._post("/systemone/noul", {
            "state": state,
            "candidates": candidates
        })
