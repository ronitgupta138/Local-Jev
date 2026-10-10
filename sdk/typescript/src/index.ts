export interface ChoiceResponse {
  choice: string;
  confidence: number;
  latency_ms: number;
  cached: boolean;
}

export class JevClient {
  constructor(private readonly baseUrl = "http://localhost:8080/v1") {}

  private async post<T>(path: string, body: Record<string, any>): Promise<T> {
    const res = await fetch(`${this.baseUrl}${path}`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body),
    });
    if (!res.ok) {
      throw new Error(`local-jev request failed (${res.status}): ${await res.text()}`);
    }
    return res.json() as Promise<T>;
  }

  public async choice(state: string, question: string, options: string[], temperature = 0.1): Promise<ChoiceResponse> {
    return this.post<ChoiceResponse>("/systemone/choice", { state, question, options, temperature });
  }

  public async score(text: string, criteria: string): Promise<number> {
    const res = await this.post<{ score: number }>("/systemone/score", { text, criteria });
    return res.score;
  }

  public async route(prompt: string, routes: string[]): Promise<string> {
    const res = await this.post<{ route: string }>("/systemone/route", { prompt, routes });
    return res.route;
  }
}
