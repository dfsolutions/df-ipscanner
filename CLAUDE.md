# IP Scanner

## Stack
- Linguaggio: Rust
- Runtime: Tokio (async)
- Package manager: Cargo

## Comandi principali
- Build: `cargo build --release`
- Test: `cargo test`
- Lint: `cargo clippy`
- Run: `cargo run -- <SUBNET> [OPTIONS]`

## Utilizzo
```bash
# Scan base (IP + MAC)
ip-scanner 192.168.1.0/24

# Con timeout custom
ip-scanner 192.168.1.0/24 -t 2

# Con hostname e OS detection
ip-scanner 192.168.1.0/24 -d

# Con port scan
ip-scanner 192.168.1.0/24 -p

# Completo
ip-scanner 192.168.1.0/24 -t 2 -d -p

# Output JSON
ip-scanner 192.168.1.0/24 -d -o json
```

## Struttura
- `src/` → codice sorgente principale
  - `cli.rs` → parsing argomenti CLI
  - `scanner/` → ping, arp, dns, port
  - `network/` → parsing CIDR
  - `os_detect/` → OS detection da TTL
  - `output/` → formatter tabella/JSON

## Convenzioni codice
- Nomi file: snake_case
- Nomi variabili/funzioni: snake_case
- Nomi tipi/struct: PascalCase
- Moduli pubblici via mod.rs

## Cosa NON fare
- Non usare unwrap() senza motivo (usa ? o expect)
- Non committare senza test
- Non installare dipendenze senza conferma esplicita

## Note Windows
- Richiede privilegi admin per ICMP raw
- ARP via SendARP (no Npcap necessario)
- Firewall potrebbe bloccare ICMP

## Ambiente Windows
- Sistema: Windows con Git Bash
- Path nei file tool: usa backslash (C:\...)
- Path nei comandi bash: usa forward slash (/c/...)
- SSH: usa ssh.exe dal PATH, non /usr/bin/ssh

## Lingua
- Commenti nel codice: inglese
- Messaggi e spiegazioni: italiano

## Piano di lavoro
Per lo stato corrente del progetto, leggi plan.md
