# CLAUDE.md

@AGENTS.md

## Precedence over global rules
This file takes precedence over `~/.claude/CLAUDE.md` wherever the two conflict.
The following global rules are tooling/workflow conventions and may be overridden
per-project; none are currently overridden here, so the global default applies:
- Git worktree usage for parallel branch work
- SQL/migration ownership (writing and running queries directly)
- Independent code-review subagent requirement


## Bakım sınırı

Depoya özgü bütün talimat gövdesi `AGENTS.md` içindedir ve yukarıdaki import ile
yüklenir. Burada yalnız Claude'a özgü davranış tutulur; ortak kural iki dosyaya
birden yazılmaz — kopya tutmak yerine tek kaynak referanslanır.
