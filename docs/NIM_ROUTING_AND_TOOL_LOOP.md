# NIM routing and tool-loop contract

Created: 2026-07-09

## Core rule

NIM is an OpenAI-compatible model provider path. It is not the agent and it is not the runtime.

The Rust runtime owns:

- state,
- objectives,
- tool eligibility,
- permission gates,
- retries,
- provider fallback,
- proof,
- final claim verification.

The model only proposes text and tool calls.

## Failure classes

### Provider/model failures

These may trigger NIM model fallback:

| Failure | First action | Fallback action |
|---|---|---|
| HTTP 429 | mark model/provider cooldown | try next configured NIM model |
| HTTP 5xx | retry once with jitter | then try next configured NIM model |
| timeout | retry once if budget remains | then try next configured NIM model |
| empty response | retry once | then try next configured NIM model |
| unavailable model | mark unavailable | try next configured NIM model |
| malformed response envelope | retry once | then try next model |
| malformed tool call | repair once using same model | then try next tool-capable NIM model |
| no tool call when one is required | correction turn once | then try next tool-capable NIM model |

### Runtime/tool failures

These must not trigger provider fallback by themselves:

| Failure | Action |
|---|---|
| shell command exits nonzero | record tool result with exit code; model must decide next step |
| file not found | record tool result; model must choose another path or report blocker |
| path denied | record permission failure; do not retry blindly |
| git dirty | record git status; model must decide whether to proceed |
| tests fail | record test output; model must inspect and fix |
| browser screenshot fails | record proof failure; runtime may retry proof path, but not route model as if provider failed |
| verifier rejects final answer | force correction from ledger, not provider fallback |

## Route engine

Input:

```rust
struct RouteRequest {
    run_id: RunId,
    turn_id: TurnId,
    purpose: RoutePurpose,
    messages: Vec<ModelMessage>,
    tools: Vec<ToolSchema>,
    require_tool_call: bool,
    budget: RouteBudget,
}
```

Output:

```rust
struct RouteOutcome {
    selected_provider: ProviderId,
    selected_model: ModelId,
    attempts: Vec<RouteAttempt>,
    response: NormalizedModelResponse,
}
```

`RoutePurpose` examples:

```text
planning
normal_turn
required_tool_turn
tool_call_repair
final_summary
claim_correction
```

## Deterministic order

Model order comes from config and is not secretly mutated.

Allowed state mutation:

- temporary cooldown records,
- per-run route attempts,
- visible provider/model failure events.

Forbidden:

- saving a new global model order based on transient success,
- hidden local fallback,
- hidden non-NIM fallback in NIM-only mode,
- capability router that silently overrides user order before MVP.

## Tool-call handling

### Accepted tool-call formats

The normalizer should support:

1. OpenAI `tool_calls` array.
2. JSON content block containing a single tool call, only if strict schema validates.
3. Streaming deltas that assemble into a tool call.

The normalizer must reject:

- partial JSON after stream end,
- unknown tool names,
- arguments that do not match schema,
- multiple write tools in one turn unless batch approval exists.

### Repair rules

Malformed tool call repair is bounded:

1. same model receives a compact correction request,
2. original malformed payload is included as data, not as trusted instruction,
3. repair gets one attempt,
4. if repair fails, route engine tries next configured tool-capable NIM model,
5. after configured budget is exhausted, session enters `failed` with proof.

## Agent loop

Pseudocode:

```rust
while session.not_final() {
    session.check_budget()?;
    session.check_loop_detector()?;

    let objective = session.next_unproven_objective();
    let route = router.complete(RouteRequest::for_objective(objective));
    proof.record_route(&route);

    let response = model_contract.normalize(route.response)?;

    if response.has_tool_calls() {
        let calls = tool_validator.validate(response.tool_calls)?;
        let results = tool_executor.execute(calls)?;
        proof.record_tool_results(results);
        session.record_observations(results);
        continue;
    }

    if session.requires_tool_call_now() {
        session.force_required_tool_correction();
        continue;
    }

    if response.is_final_candidate() {
        let verdict = claim_verifier.verify(response.text, proof.ledger());
        if verdict.pass {
            session.finalize(response.text);
        } else {
            session.force_final_correction(verdict);
        }
    }
}
```

## Loop detector

Detect repeated patterns across a sliding window:

- same tool name + same normalized args,
- same failed command repeated without inspecting output,
- same final claim rejected twice,
- same objective reopened without new evidence,
- turn count high with no new proof events.

Intervention:

1. summarize current ledger,
2. list unchecked objectives,
3. forbid repeating last failed action unchanged,
4. request one concrete next action.

## Six-phase benchmark handling

The six-phase benchmark is not special-cased.

The runtime should parse it into objectives:

```text
phase_1_repo_inspection
phase_2_long_tool_loop
phase_3_file_write_read_delete
phase_4_analysis_report
phase_5_validation
phase_6_final_summary
```

Each objective has evidence requirements.

Example:

```json
{
  "objective_id": "phase_1_repo_inspection",
  "requires": [
    "pwd_observed",
    "git_root_observed",
    "branch_observed",
    "head_observed",
    "remote_observed",
    "dirty_state_observed",
    "build_system_observed",
    "test_commands_observed"
  ]
}
```

The final response is accepted only if all required evidence exists in the ledger.

## Why previous NIM runs looked broken

Likely root causes from previous project history:

1. Model was allowed to own too much progress state.
2. Long prompts depended on context memory instead of objective ledger.
3. Tool failures and provider failures were blurred.
4. Browser proof lived beside the run instead of inside the run data model.
5. Benchmark pass/fail could be proven by checker artifacts while the user still lacked trustworthy live UI proof.
6. Huge branches made it impossible to tell which behavior was current.

The fix is not more fallback. The fix is a smaller stateful runtime.
