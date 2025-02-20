# QuantumVault - Password Manager

QuantumVault è un gestore di password CLI (Command-Line Interface) realizzato in Rust. L'applicazione è progettato per memorizzare e gestire in modo sicuro le tue credenziali, proteggendo i tuoi dati con crittografia avanzata.

## Caratteristiche principali

- **Gestione di credenziali**: Aggiungi, visualizza, modifica e rimuovi login per diversi servizi
- **Crittografia sicura**: 
  - Utilizza AES-256-GCM per crittografare le password
  - Implementa Argon2 per l'hashing della password master
- **Interfaccia utente**: Menu intuitivo per navigare tra le diverse funzionalità
- **Integrazione con clipboard**: Copia automaticamente le password negli appunti
- **Salvataggio automatico**: I dati vengono salvati in una posizione sicura nel file system

## Installazione

1. Clona il repository:
```bash
git clone https://github.com/tuo-username/quantumvault.git
```

2. Compila il progetto:
```bash
cargo build
```

3. Esegui il programma:
```bash
cargo run -- nuovo-vault
```

## Utilizzo

### Comandi disponibili

- `quantumvault nuovo-vault`: Crea un nuovo vault
- `quantumvault login`: Accedi a un vault esistente
- `quantumvault elimina-vault`: Elimina un vault

### Menu principale

Una volta acceso al vault, potrai:

- Aggiungere nuove credenziali
- Visualizzare le credenziali salvate
- Rimuovere credenziali
- Modificare informazioni esistenti
- Copiare automaticamente le password negli appunti
- Tornare al menu principale
- Esci dal programma

## Sicurezza

- Le password vengono crittografate con AES-256-GCM
- La password master viene hashata con Argon2
- I dati vengono salvati localmente in un file crittografato
- Nessun dato viene inviato su internet
- La password master non viene mai memorizzata


## Note

- Assicurati di mantenere al sicuro la tua password master, poiché non c'è possibilità di recupero
- I dati vengono salvati nella cartella `~/.local/share/.password_manager` su windows oppure `~/Library/Application Support/.password_manager`
