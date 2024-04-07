# ByteLang
wip Dev Bytes discord programming language impl

## Design

### Paradigm
- Procedural

### Type System
- Statically Typed
- Strongly Typed
- Null

## Impl
### Architecture
- Recursive Descent Parser
- Flat, Variable-Length Instruction IR
- Stack-Based Interpreter

### Memory
- Primitives are stack allocated. User-Defined Types are heap allocated, barring escape analysis
- Tracing Garbage Collector
