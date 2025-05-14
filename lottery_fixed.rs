// Importazione delle librerie necessarie
use anchor_lang::prelude::*;             // Importa tutte le funzionalità principali del framework Anchor
use borsh::{BorshDeserialize, BorshSerialize};  // Importa funzionalità per serializzare/deserializzare dati
use anchor_lang::system_program;         // Importa le funzionalità del programma di sistema di Solana

// Dichiarazione dell'ID del programma - questo è l'identificativo univoco del programma sulla blockchain Solana
declare_id!("8YPpeioVZKZAXGEogghYyKfCpBJruXQUWZuwoUawFczB");

// Costante che definisce l'estensione della scadenza in numero di slot
const DEADLINE_EXTENSION: u64 = 10;

// Definizione del modulo del programma - qui vengono definite tutte le funzioni che possono essere chiamate dall'esterno
#[program]
pub mod lottery {
    use super::*;  // Importa tutto dal modulo padre

    // Funzione per iniziare la lotteria
    pub fn join(
        ctx: Context<JoinCtx>,        // Contesto che contiene gli account coinvolti
        hashlock1: [u8; 32],          // Hash del segreto del giocatore 1 (32 byte)
        hashlock2: [u8; 32],          // Hash del segreto del giocatore 2 (32 byte)
        delay: u64,                   // Ritardo prima della fine della fase di rivelazione
        amount: u64,                  // Importo da scommettere
    ) -> Result<()> {                 // Ritorna un Result, che può essere Ok o un errore
        let lottery_info = &mut ctx.accounts.lottery_info;  // Ottiene un riferimento mutabile all'account della lotteria
        let end_reveal = Clock::get()?.slot + delay;        // Calcola quando finirà la fase di rivelazione
        
        // Inizializza l'account della lotteria con i dati forniti
        lottery_info.initialize(
            *ctx.accounts.player1.key,    // Chiave pubblica del giocatore 1
            *ctx.accounts.player2.key,    // Chiave pubblica del giocatore 2
            hashlock1,                   // Hash del segreto del giocatore 1
            hashlock2,                   // Hash del segreto del giocatore 2
            end_reveal,                  // Slot di fine rivelazione
        )?;

        // Trasferisce l'importo dal giocatore 1 all'account della lotteria
        let player1 = ctx.accounts.player1.to_account_info();
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: player1.clone(),                       // Da: giocatore 1
                    to: lottery_info.to_account_info().clone(),  // A: account lotteria
                },
            ),
            amount,  // Importo da trasferire
        )?;

        // Trasferisce l'importo dal giocatore 2 all'account della lotteria
        let player2 = ctx.accounts.player2.to_account_info();
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: player2.clone(),                       // Da: giocatore 2
                    to: lottery_info.to_account_info().clone(),  // A: account lotteria
                },
            ),
            amount,  // Importo da trasferire
        )?;

        Ok(())  // Ritorna successo
    }

    // Funzione per il giocatore 1 per rivelare il suo segreto
    pub fn reveal_p1(ctx: Context<RevealP1Ctx>, secret: String) -> Result<()> {
        let lottery_info = &mut ctx.accounts.lottery_info;  // Ottiene un riferimento mutabile all'account della lotteria
        lottery_info.reveal_p1(&secret)?;  // Chiama il metodo per rivelare il segreto del giocatore 1
        Ok(())  // Ritorna successo
    }

    // Funzione per il giocatore 2 per rivelare il suo segreto
    pub fn reveal_p2(ctx: Context<RevealP2Ctx>, secret: String) -> Result<()> {
        let lottery_info = &mut ctx.accounts.lottery_info;  // Ottiene un riferimento mutabile all'account della lotteria
        lottery_info.reveal_p2(&secret)?;  // Chiama il metodo per rivelare il segreto del giocatore 2

        let winner = lottery_info.get_winner()?;  // Determina il vincitore

        // Trasferisce tutto il denaro al vincitore
        if winner == *ctx.accounts.player1.key {
            // Se il vincitore è il giocatore 1, trasferisce i lamports a lui
            let player1 = &ctx.accounts.player1;
            **player1.to_account_info().try_borrow_mut_lamports()? +=
                lottery_info.to_account_info().lamports();
        } else {
            // Altrimenti, trasferisce i lamports al giocatore 2
            let player2 = &ctx.accounts.player2;
            **player2.to_account_info().try_borrow_mut_lamports()? +=
                lottery_info.to_account_info().lamports();
        }
        // Azzera il saldo dell'account lotteria
        **lottery_info.to_account_info().try_borrow_mut_lamports()? = 0;
        Ok(())  // Ritorna successo
    }

    // Funzione che permette al giocatore 2 di riscattare il premio se il giocatore 1 non rivela il suo segreto in tempo
    pub fn redeem_if_p1_no_reveal(ctx: Context<RedeemIfP1NoRevealCtx>) -> Result<()> {
        let lottery_info = &mut ctx.accounts.lottery_info;  // Ottiene un riferimento mutabile all'account della lotteria
        lottery_info.check_redeem_if_p1_no_reveal()?;  // Verifica che si possa riscattare
        
        // Trasferisce tutti i lamports al giocatore 2
        let player2 = &ctx.accounts.player2;
        **player2.to_account_info().try_borrow_mut_lamports()? +=
            lottery_info.to_account_info().lamports();
        **lottery_info.to_account_info().try_borrow_mut_lamports()? = 0;  // Azzera il saldo dell'account lotteria
        Ok(())  // Ritorna successo
    }

    // Funzione che permette al giocatore 1 di riscattare il premio se il giocatore 2 non rivela il suo segreto in tempo
    pub fn redeem_if_p2_no_reveal(ctx: Context<RedeemIfP2NoRevealCtx>) -> Result<()> {
        let lottery_info = &mut ctx.accounts.lottery_info;  // Ottiene un riferimento mutabile all'account della lotteria
        lottery_info.check_redeem_if_p2_no_reveal()?;  // Verifica che si possa riscattare
        
        // Trasferisce tutti i lamports al giocatore 1
        let player1 = &ctx.accounts.player1;
        **player1.to_account_info().try_borrow_mut_lamports()? +=
            lottery_info.to_account_info().lamports();
        **lottery_info.to_account_info().try_borrow_mut_lamports()? = 0;  // Azzera il saldo dell'account lotteria
        Ok(())  // Ritorna successo
    }
}

// Definizione dello stato della lotteria come enum
#[derive( Debug, PartialEq, Clone, AnchorSerialize, 
    AnchorDeserialize )]
pub enum LotteryState {
    Init = 0,       // Stato iniziale
    RevealP1 = 1,   // Giocatore 1 ha rivelato
    RevealP2 = 2,   // Giocatore 2 ha rivelato
}

use anchor_lang::Space;

// Implementazione del trait Space per LotteryState (determina quanto spazio allocare)
impl Space for LotteryState {
    const INIT_SPACE: usize = 1; // 1 byte per l'enum (0-2)
}

// Definizione della struttura dati principale che memorizza le informazioni della lotteria
#[account]
#[derive(InitSpace)]
pub struct LotteryInfo {
    pub state: LotteryState,      // Stato attuale della lotteria
    pub player1: Pubkey,          // Chiave pubblica del giocatore 1
    pub player2: Pubkey,          // Chiave pubblica del giocatore 2
    pub hashlock1: [u8; 32],      // Hash del segreto del giocatore 1
    #[max_len(30)]                // Lunghezza massima del segreto: 30 caratteri
    pub secret1: String,          // Segreto rivelato dal giocatore 1
    pub hashlock2: [u8; 32],      // Hash del segreto del giocatore 2
    #[max_len(30)]                // Lunghezza massima del segreto: 30 caratteri
    pub secret2: String,          // Segreto rivelato dal giocatore 2
    pub end_reveal: u64,          // Slot di fine della fase di rivelazione
}

// Implementazione dei metodi per la struttura LotteryInfo
impl LotteryInfo {
    // Metodo per inizializzare la lotteria
    pub fn initialize(
        &mut self,                // Riferimento mutabile a sé stesso
        player1: Pubkey,          // Chiave pubblica del giocatore 1
        player2: Pubkey,          // Chiave pubblica del giocatore 2
        hashlock1: [u8; 32],      // Hash del segreto del giocatore 1
        hashlock2: [u8; 32],      // Hash del segreto del giocatore 2
        end_reveal: u64,          // Slot di fine rivelazione
    ) -> Result<()> {
        // Verifica che i due hash siano diversi
        require!(hashlock1 != hashlock2, CustomError::TwoEqualHashes);
        // Verifica che lo slot di fine rivelazione sia nel futuro
        require!(
            Clock::get()?.slot < end_reveal,
            CustomError::InvalidTimeoutProvided
        );
        
        // Inizializza i campi della struttura
        self.state = LotteryState::Init;
        self.player1 = player1;
        self.player2 = player2;
        self.hashlock1 = hashlock1;
        self.hashlock2 = hashlock2;
        self.end_reveal = end_reveal;
        Ok(())  // Ritorna successo
    }

    // Metodo per il giocatore 1 per rivelare il suo segreto
    pub fn reveal_p1(&mut self, secret: &String) -> Result<()> {
        // Verifica che lo stato sia quello iniziale
        require!(self.state == LotteryState::Init, CustomError::InvalidState);
        // Verifica che non sia scaduto il tempo per rivelare
        require!(
            Clock::get()?.slot < self.end_reveal,
            CustomError::TimeoutReached
        );
        
        // Calcola l'hash del segreto fornito
        let hash = anchor_lang::solana_program::keccak::hash(
            &<String as Clone>::clone(&secret).into_bytes(),
        )
        .to_bytes();
        
        // Verifica che l'hash corrisponda a quello memorizzato
        require!(hash == self.hashlock1, CustomError::InvalidSecret);
        
        // Memorizza il segreto e aggiorna lo stato
        self.secret1 = secret.clone();
        self.state = LotteryState::RevealP1;
        Ok(())  // Ritorna successo
    }

    // Metodo per il giocatore 2 per rivelare il suo segreto
    pub fn reveal_p2(&mut self, secret: &String) -> Result<()> {
        // Verifica che il giocatore 1 abbia già rivelato
        require!(
            self.state == LotteryState::RevealP1,
            CustomError::InvalidState
        );
        // Verifica che non sia scaduto il tempo per rivelare (con estensione)
        require!(
            Clock::get()?.slot < self.end_reveal + DEADLINE_EXTENSION,
            CustomError::InvalidTimeoutProvided
        );
        
        // Calcola l'hash del segreto fornito
        let hash = anchor_lang::solana_program::keccak::hash(
            &<String as Clone>::clone(&secret).into_bytes(),
        )
        .to_bytes();
        
        // Verifica che l'hash corrisponda a quello memorizzato
        require!(hash == self.hashlock2, CustomError::InvalidSecret);
        
        // Memorizza il segreto e aggiorna lo stato
        self.secret2 = secret.clone();
        self.state = LotteryState::RevealP2;
        Ok(())  // Ritorna successo
    }

    // Metodo per determinare il vincitore
    pub fn get_winner(&self) -> Result<Pubkey> {
        // Verifica che entrambi i giocatori abbiano rivelato i loro segreti
        require!(
            self.state == LotteryState::RevealP2,
            CustomError::InvalidState
        );
        
        // Calcola la somma delle lunghezze dei segreti
        let sum = self.secret1.len() + self.secret2.len();
        
        // Determina il vincitore in base alla parità della somma
        if sum % 2 == 0 {
            // Se la somma è pari, vince il giocatore 1
            Ok(self.player1)
        } else {
            // Se la somma è dispari, vince il giocatore 2
            Ok(self.player2)
        }
    }

    // Metodo per verificare che si possa riscattare se il giocatore 1 non ha rivelato
    pub fn check_redeem_if_p1_no_reveal(&self) -> Result<()> {
        // Verifica che siamo ancora nello stato iniziale (giocatore 1 non ha rivelato)
        require!(self.state == LotteryState::Init, CustomError::InvalidState);
        // Verifica che sia passato il tempo per rivelare
        require!(
            Clock::get()?.slot > self.end_reveal,
            CustomError::TimeoutNotReached
        );
        Ok(())  // Ritorna successo
    }

    // Metodo per verificare che si possa riscattare se il giocatore 2 non ha rivelato
    pub fn check_redeem_if_p2_no_reveal(&self) -> Result<()> {
        // Verifica che il giocatore 1 abbia rivelato ma il 2 no
        require!(
            self.state == LotteryState::RevealP1,
            CustomError::InvalidState
        );
        // Verifica che sia passato il tempo per rivelare (con estensione)
        require!(
            Clock::get()?.slot > self.end_reveal + DEADLINE_EXTENSION,
            CustomError::TimeoutNotReached
        );
        Ok(())  // Ritorna successo
    }
}

// Definizione del contesto per la funzione join
#[derive(Accounts)]
pub struct JoinCtx<'info> {
    #[account(mut)]                    // Account che può essere modificato
    pub player1: Signer<'info>,        // Il giocatore 1 deve firmare la transazione
    #[account(mut)]                    // Account che può essere modificato
    pub player2: Signer<'info>,        // Il giocatore 2 deve firmare la transazione
    #[account(
        init,                          // Inizializza un nuovo account
        payer = player1,               // Il giocatore 1 paga per la creazione dell'account
        seeds = [player1.key().as_ref(), player2.key().as_ref()],  // Chiavi usate per derivare l'indirizzo dell'account
        bump,                          // Parametro per la derivazione dell'indirizzo
        space = 8 + LotteryInfo::INIT_SPACE  // Spazio da allocare: 8 byte di discriminatore + spazio per i dati
    )]
    pub lottery_info: Account<'info, LotteryInfo>,  // Account che conterrà i dati della lotteria
    pub system_program: Program<'info, System>,     // Programma di sistema necessario per creare account
}

// Definizione del contesto per la funzione reveal_p1
#[derive(Accounts)]
pub struct RevealP1Ctx<'info> {
    #[account(mut)]                    // Account che può essere modificato
    pub player1: Signer<'info>,        // Il giocatore 1 deve firmare la transazione
    pub player2: SystemAccount<'info>, // Account del giocatore 2 (non deve firmare)
    #[account(
        mut,                           // Account che può essere modificato
        seeds = [player1.key().as_ref(), player2.key().as_ref()],  // Chiavi usate per derivare l'indirizzo dell'account
        bump,                          // Parametro per la derivazione dell'indirizzo
    )]
    pub lottery_info: Account<'info, LotteryInfo>,  // Account che contiene i dati della lotteria
}

// Definizione del contesto per la funzione reveal_p2
#[derive(Accounts)]
pub struct RevealP2Ctx<'info> {
    #[account(mut)]                    // Account che può essere modificato
    pub player1: SystemAccount<'info>, // Account del giocatore 1 (non deve firmare)
    #[account(mut)]                    // Account che può essere modificato
    pub player2: Signer<'info>,        // Il giocatore 2 deve firmare la transazione
    #[account(
        mut,                           // Account che può essere modificato
        seeds = [player1.key().as_ref(), player2.key().as_ref()],  // Chiavi usate per derivare l'indirizzo dell'account
        bump,                          // Parametro per la derivazione dell'indirizzo
    )]
    pub lottery_info: Account<'info, LotteryInfo>,  // Account che contiene i dati della lotteria
}

// Definizione del contesto per la funzione redeem_if_p1_no_reveal
#[derive(Accounts)]
pub struct RedeemIfP1NoRevealCtx<'info> {
    pub player1: SystemAccount<'info>, // Account del giocatore 1 (non deve firmare)
    #[account(mut)]                    // Account che può essere modificato
    pub player2: Signer<'info>,        // Il giocatore 2 deve firmare la transazione
    #[account(
        mut,                           // Account che può essere modificato
        seeds = [player1.key().as_ref(), player2.key().as_ref()],  // Chiavi usate per derivare l'indirizzo dell'account
        bump,                          // Parametro per la derivazione dell'indirizzo
    )]
    pub lottery_info: Account<'info, LotteryInfo>,  // Account che contiene i dati della lotteria
}

// Definizione del contesto per la funzione redeem_if_p2_no_reveal
#[derive(Accounts)]
pub struct RedeemIfP2NoRevealCtx<'info> {
    #[account(mut)]                    // Account che può essere modificato
    pub player1: Signer<'info>,        // Il giocatore 1 deve firmare la transazione
    pub player2: SystemAccount<'info>, // Account del giocatore 2 (non deve firmare)
    #[account(
        mut,                           // Account che può essere modificato
        seeds = [player1.key().as_ref(), player2.key().as_ref()],  // Chiavi usate per derivare l'indirizzo dell'account
        bump,                          // Parametro per la derivazione dell'indirizzo
    )]
    pub lottery_info: Account<'info, LotteryInfo>,  // Account che contiene i dati della lotteria
}

// Definizione di un contesto vuoto (non utilizzato nel codice)
#[derive(Accounts)]
pub struct WinCtx {}

// Definizione degli errori personalizzati
#[error_code]
pub enum CustomError {
    #[msg("Invalid state")]             // Messaggio di errore: stato non valido
    InvalidState,

    #[msg("Invalid timeout provided")]  // Messaggio di errore: timeout non valido
    InvalidTimeoutProvided,

    #[msg("Timeout reached")]           // Messaggio di errore: timeout raggiunto
    TimeoutReached,

    #[msg("Timeout not reached")]       // Messaggio di errore: timeout non ancora raggiunto
    TimeoutNotReached,

    #[msg("Invalid secret")]            // Messaggio di errore: segreto non valido
    InvalidSecret,

    #[msg("Two equal hashes")]          // Messaggio di errore: due hash uguali
    TwoEqualHashes,
}
