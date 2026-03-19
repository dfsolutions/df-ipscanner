# Piano di lavoro — IP Scanner

## Obiettivo
Tool CLI in Rust per scansione IP di subnet. Rileva host attivi con MAC address, hostname, OS (da TTL) e porte aperte.

## Stato attuale
- [x] Setup iniziale progetto
- [x] Struttura cartelle e CLAUDE.md
- [x] Definire stack tecnologico (Rust + Tokio)
- [x] Implementare CLI parsing (clap)
- [x] Implementare CIDR parsing
- [x] Implementare ICMP ping
- [x] Implementare ARP lookup (Windows API)
- [x] Implementare DNS reverse lookup
- [x] Implementare OS detection (TTL)
- [x] Implementare port scanner
- [x] Implementare output formatter (tabella + JSON)
- [ ] Installare Rust/Cargo
- [ ] Build e test
- [ ] Deploy (release build)

## Sessione corrente
**Data:** 2026-03-18
**Focus:** Implementazione completa IP Scanner
**Dove ero rimasto:** Codice completato, in attesa installazione Rust

## Decisioni prese
- 2026-03-18: Linguaggio Rust con async Tokio
- 2026-03-18: Flag -d per detect (non -h che è riservato a help)
- 2026-03-18: Flag -p per port scan
- 2026-03-18: ARP via SendARP Windows API (no Npcap)
- 2026-03-18: OS detection via TTL response

## Problemi noti
- Richiede privilegi admin su Windows per ICMP raw socket
- Firewall Windows potrebbe bloccare ping

## Note per la prossima sessione
- Installare Rust da rustup.rs
- Eseguire `cargo build --release`
- Testare con subnet locale
