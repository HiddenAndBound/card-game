**Todo List**

- [ ] Pin Bevy version and repo structure  
Dependencies: None  
Testing criteria: `cargo check` passes; app launches to an empty window  
Description: Lock Bevy version, define module layout, and set up basic app entrypoint.

- [ ] App state machine and scene transitions  
Dependencies: Pin Bevy version and repo structure  
Testing criteria: Manual: transitions between menu, run, combat, victory, defeat without panic  
Description: Implement core game states and state transitions.

- [ ] Combat simulation core (deck/hand/draw/discard/energy/turns)  
Dependencies: Pin Bevy version and repo structure  
Testing criteria: Unit tests for draw/discard, energy reset, damage resolution; manual: simulated combat loop works  
Description: Build the pure Rust combat model independent of UI.

- [ ] Bevy UI scaffolding for combat HUD  
Dependencies: App state machine and scene transitions  
Testing criteria: Manual: UI renders hand area, health, energy, end-turn button  
Description: Create basic layout and widgets for the combat UI.

- [ ] TOML schema for cards and enemies  
Dependencies: Combat simulation core (deck/hand/draw/discard/energy/turns)  
Testing criteria: Schema documented and reviewed; sample files load successfully  
Description: Define data fields for cards and enemies to support core mechanics.

- [ ] TOML loader with validation and error reporting  
Dependencies: TOML schema for cards and enemies  
Testing criteria: Invalid TOML yields readable errors; valid files load deterministically  
Description: Implement data loading pipeline and validation for game content.

- [ ] Minimal debug hooks (start combat, restart run)  
Dependencies: App state machine and scene transitions  
Testing criteria: Manual: can start a run and reset without restarting the app  
Description: Add basic debug actions to speed up iteration.

- [ ] Fixed starter deck and initial card set (8–12 cards)  
Dependencies: TOML loader with validation and error reporting  
Testing criteria: Manual: cards load and appear in hand during combat  
Description: Create a minimal set of cards for MVP.

- [ ] Combat UI actions (play card, end turn, draw)  
Dependencies: Combat simulation core (deck/hand/draw/discard/energy/turns), Bevy UI scaffolding for combat HUD  
Testing criteria: Manual: play cards affects state; end-turn resolves enemy turn  
Description: Wire UI controls to combat simulation.

- [ ] Encounter loop with 3 fixed fights  
Dependencies: Combat UI actions (play card, end turn, draw), TOML loader with validation and error reporting  
Testing criteria: Manual: can progress through 3 fights and reach end state  
Description: Implement fixed encounter sequence for MVP run.

- [ ] Victory/defeat screens and run restart  
Dependencies: Encounter loop with 3 fixed fights  
Testing criteria: Manual: win and lose both show correct screens; restart works  
Description: Add end-state UI and loop-back flow.

- [ ] Enemy intents and basic AI sequencing  
Dependencies: Combat simulation core (deck/hand/draw/discard/energy/turns)  
Testing criteria: Manual: enemy actions follow intent pattern across turns  
Description: Implement deterministic enemy behavior for MVP.

- [ ] UX readability pass (card layout, turn feedback)  
Dependencies: Combat UI actions (play card, end turn, draw)  
Testing criteria: Manual: testers can identify health, energy, and turn status quickly  
Description: Improve clarity and affordance of combat UI.

- [ ] TOML encounter definitions and loader  
Dependencies: TOML loader with validation and error reporting  
Testing criteria: Manual: encounters are configured via TOML without code changes  
Description: Move encounter data to TOML for faster iteration.

- [ ] Reward flow between combats  
Dependencies: Encounter loop with 3 fixed fights  
Testing criteria: Manual: post-fight reward appears and updates deck  
Description: Add simple reward step between fights.

- [ ] Content validation improvements  
Dependencies: TOML loader with validation and error reporting  
Testing criteria: Invalid field types and missing keys are reported clearly  
Description: Expand validation for better content authoring feedback.

- [ ] Balance pass (damage/health/energy)  
Dependencies: Fixed starter deck and initial card set (8–12 cards), Enemy intents and basic AI sequencing  
Testing criteria: Manual: average run is winnable in 3 fights without obvious exploits  
Description: Tune numbers for a fun MVP.

- [ ] Run summary screen  
Dependencies: Victory/defeat screens and run restart  
Testing criteria: Manual: summary shows outcome and basic stats; restart works  
Description: Add simple end-of-run recap.

- [ ] Debug utilities (skip encounter, add card, set HP)  
Dependencies: Minimal debug hooks (start combat, restart run)  
Testing criteria: Manual: debug actions work without corrupting game state  
Description: Add quick tools to speed testing and balance iterations.

- [ ] Playtest pass and bug fixes  
Dependencies: Run summary screen, Balance pass (damage/health/energy)  
Testing criteria: 10 consecutive runs without crashes; known issues logged  
Description: Consolidate stability and fix critical issues.

- [ ] Optional minimal SFX placeholders  
Dependencies: Combat UI actions (play card, end turn, draw)  
Testing criteria: Manual: SFX triggers on play/end/victory without audio errors  
Description: Add lightweight audio feedback if time allows.

- [ ] Local LLM runtime integration (llama.cpp + gguf)  
Dependencies: Playtest pass and bug fixes  
Testing criteria: Model loads offline; response returned within acceptable latency  
Description: Add local LLM runtime for GM commentary.

- [ ] Prompt templates and response parsing  
Dependencies: Local LLM runtime integration (llama.cpp + gguf)  
Testing criteria: Manual: prompts yield bounded, parseable output  
Description: Create structured prompts and parse responses safely.

- [ ] Safe effect constraints and validation  
Dependencies: Prompt templates and response parsing  
Testing criteria: Manual: LLM outputs cannot break game rules or crash  
Description: Gate and validate any LLM-driven state changes.

- [ ] Fallback scripted commentary  
Dependencies: Prompt templates and response parsing  
Testing criteria: Manual: game continues when LLM fails or is unavailable  
Description: Add deterministic fallback text.

- [ ] Performance and latency tuning  
Dependencies: Local LLM runtime integration (llama.cpp + gguf)  
Testing criteria: Manual: acceptable response time during combat; no major frame drops  
Description: Optimize model size, batching, and call frequency.
