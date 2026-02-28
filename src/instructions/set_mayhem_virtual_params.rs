use borsh::BorshDeserialize;
use borsh::BorshSerialize;

pub const SET_MAYHEM_VIRTUAL_PARAMS_DISCRIMINATOR: [u8; 8] =
    [61, 169, 188, 191, 153, 149, 42, 97];

#[derive(Debug)]
pub struct SetMayhemVirtualParams {
    pub sol_vault_authority: solana_pubkey::Pubkey,

    pub mayhem_token_vault: solana_pubkey::Pubkey,

    pub mint: solana_pubkey::Pubkey,

    pub global: solana_pubkey::Pubkey,

    pub bonding_curve: solana_pubkey::Pubkey,

    pub token_program: solana_pubkey::Pubkey,

    pub event_authority: solana_pubkey::Pubkey,

    pub program: solana_pubkey::Pubkey,
}

impl SetMayhemVirtualParams {
    pub fn instruction(&self) -> solana_instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_instruction::AccountMeta],
    ) -> solana_instruction::Instruction {
        let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new(
            self.sol_vault_authority,
            true,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            self.mayhem_token_vault,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.mint,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.global,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            self.bonding_curve,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.event_authority,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = SetMayhemVirtualParamsInstructionData::new()
            .try_to_vec()
            .unwrap();

        solana_instruction::Instruction {
            program_id: crate::PUMP_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetMayhemVirtualParamsInstructionData {
    discriminator: [u8; 8],
}

impl SetMayhemVirtualParamsInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [61, 169, 188, 191, 153, 149, 42, 97],
        }
    }

    pub(crate) fn try_to_vec(&self) -> Result<Vec<u8>, std::io::Error> {
        borsh::to_vec(self)
    }
}

impl Default for SetMayhemVirtualParamsInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Default)]
pub struct SetMayhemVirtualParamsBuilder {
    sol_vault_authority: Option<solana_pubkey::Pubkey>,
    mayhem_token_vault: Option<solana_pubkey::Pubkey>,
    mint: Option<solana_pubkey::Pubkey>,
    global: Option<solana_pubkey::Pubkey>,
    bonding_curve: Option<solana_pubkey::Pubkey>,
    token_program: Option<solana_pubkey::Pubkey>,
    event_authority: Option<solana_pubkey::Pubkey>,
    program: Option<solana_pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl SetMayhemVirtualParamsBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn sol_vault_authority(
        &mut self,
        sol_vault_authority: solana_pubkey::Pubkey,
    ) -> &mut Self {
        self.sol_vault_authority = Some(sol_vault_authority);
        self
    }
    #[inline(always)]
    pub fn mayhem_token_vault(
        &mut self,
        mayhem_token_vault: solana_pubkey::Pubkey,
    ) -> &mut Self {
        self.mayhem_token_vault = Some(mayhem_token_vault);
        self
    }
    #[inline(always)]
    pub fn mint(&mut self, mint: solana_pubkey::Pubkey) -> &mut Self {
        self.mint = Some(mint);
        self
    }
    #[inline(always)]
    pub fn global(&mut self, global: solana_pubkey::Pubkey) -> &mut Self {
        self.global = Some(global);
        self
    }
    #[inline(always)]
    pub fn bonding_curve(&mut self, bonding_curve: solana_pubkey::Pubkey) -> &mut Self {
        self.bonding_curve = Some(bonding_curve);
        self
    }
    /// `[optional account, default to 'TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn event_authority(&mut self, event_authority: solana_pubkey::Pubkey) -> &mut Self {
        self.event_authority = Some(event_authority);
        self
    }
    #[inline(always)]
    pub fn program(&mut self, program: solana_pubkey::Pubkey) -> &mut Self {
        self.program = Some(program);
        self
    }
    #[inline(always)]
    pub fn add_remaining_account(&mut self, account: solana_instruction::AccountMeta) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_instruction::Instruction {
        let accounts = SetMayhemVirtualParams {
            sol_vault_authority: self
                .sol_vault_authority
                .expect("sol_vault_authority is not set"),
            mayhem_token_vault: self
                .mayhem_token_vault
                .expect("mayhem_token_vault is not set"),
            mint: self.mint.expect("mint is not set"),
            global: self.global.expect("global is not set"),
            bonding_curve: self.bonding_curve.expect("bonding_curve is not set"),
            token_program: self
                .token_program
                .unwrap_or(solana_pubkey::pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb")),
            event_authority: self.event_authority.expect("event_authority is not set"),
            program: self.program.expect("program is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

pub struct SetMayhemVirtualParamsCpiAccounts<'a, 'b> {
    pub sol_vault_authority: &'b solana_account_info::AccountInfo<'a>,

    pub mayhem_token_vault: &'b solana_account_info::AccountInfo<'a>,

    pub mint: &'b solana_account_info::AccountInfo<'a>,

    pub global: &'b solana_account_info::AccountInfo<'a>,

    pub bonding_curve: &'b solana_account_info::AccountInfo<'a>,

    pub token_program: &'b solana_account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_account_info::AccountInfo<'a>,

    pub program: &'b solana_account_info::AccountInfo<'a>,
}

pub struct SetMayhemVirtualParamsCpi<'a, 'b> {
    pub __program: &'b solana_account_info::AccountInfo<'a>,

    pub sol_vault_authority: &'b solana_account_info::AccountInfo<'a>,

    pub mayhem_token_vault: &'b solana_account_info::AccountInfo<'a>,

    pub mint: &'b solana_account_info::AccountInfo<'a>,

    pub global: &'b solana_account_info::AccountInfo<'a>,

    pub bonding_curve: &'b solana_account_info::AccountInfo<'a>,

    pub token_program: &'b solana_account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_account_info::AccountInfo<'a>,

    pub program: &'b solana_account_info::AccountInfo<'a>,
}

impl<'a, 'b> SetMayhemVirtualParamsCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_account_info::AccountInfo<'a>,
        accounts: SetMayhemVirtualParamsCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            sol_vault_authority: accounts.sol_vault_authority,
            mayhem_token_vault: accounts.mayhem_token_vault,
            mint: accounts.mint,
            global: accounts.global,
            bonding_curve: accounts.bonding_curve,
            token_program: accounts.token_program,
            event_authority: accounts.event_authority,
            program: accounts.program,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program_error::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(&'b solana_account_info::AccountInfo<'a>, bool, bool)],
    ) -> solana_program_error::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program_error::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(&'b solana_account_info::AccountInfo<'a>, bool, bool)],
    ) -> solana_program_error::ProgramResult {
        let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new(
            *self.sol_vault_authority.key,
            true,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.mayhem_token_vault.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.mint.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.global.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.bonding_curve.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.event_authority.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = SetMayhemVirtualParamsInstructionData::new()
            .try_to_vec()
            .unwrap();

        let instruction = solana_instruction::Instruction {
            program_id: crate::PUMP_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(9 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.sol_vault_authority.clone());
        account_infos.push(self.mayhem_token_vault.clone());
        account_infos.push(self.mint.clone());
        account_infos.push(self.global.clone());
        account_infos.push(self.bonding_curve.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.event_authority.clone());
        account_infos.push(self.program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_cpi::invoke(&instruction, &account_infos)
        } else {
            solana_cpi::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

#[derive(Clone, Debug)]
pub struct SetMayhemVirtualParamsCpiBuilder<'a, 'b> {
    instruction: Box<SetMayhemVirtualParamsCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> SetMayhemVirtualParamsCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(SetMayhemVirtualParamsCpiBuilderInstruction {
            __program: program,
            sol_vault_authority: None,
            mayhem_token_vault: None,
            mint: None,
            global: None,
            bonding_curve: None,
            token_program: None,
            event_authority: None,
            program: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn sol_vault_authority(
        &mut self,
        sol_vault_authority: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.sol_vault_authority = Some(sol_vault_authority);
        self
    }
    #[inline(always)]
    pub fn mayhem_token_vault(
        &mut self,
        mayhem_token_vault: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.mayhem_token_vault = Some(mayhem_token_vault);
        self
    }
    #[inline(always)]
    pub fn mint(&mut self, mint: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.mint = Some(mint);
        self
    }
    #[inline(always)]
    pub fn global(&mut self, global: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.global = Some(global);
        self
    }
    #[inline(always)]
    pub fn bonding_curve(
        &mut self,
        bonding_curve: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.bonding_curve = Some(bonding_curve);
        self
    }
    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn event_authority(
        &mut self,
        event_authority: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.event_authority = Some(event_authority);
        self
    }
    #[inline(always)]
    pub fn program(&mut self, program: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.program = Some(program);
        self
    }
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(&'b solana_account_info::AccountInfo<'a>, bool, bool)],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program_error::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program_error::ProgramResult {
        let instruction = SetMayhemVirtualParamsCpi {
            __program: self.instruction.__program,

            sol_vault_authority: self
                .instruction
                .sol_vault_authority
                .expect("sol_vault_authority is not set"),

            mayhem_token_vault: self
                .instruction
                .mayhem_token_vault
                .expect("mayhem_token_vault is not set"),

            mint: self.instruction.mint.expect("mint is not set"),

            global: self.instruction.global.expect("global is not set"),

            bonding_curve: self
                .instruction
                .bonding_curve
                .expect("bonding_curve is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            event_authority: self
                .instruction
                .event_authority
                .expect("event_authority is not set"),

            program: self.instruction.program.expect("program is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct SetMayhemVirtualParamsCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_account_info::AccountInfo<'a>,
    sol_vault_authority: Option<&'b solana_account_info::AccountInfo<'a>>,
    mayhem_token_vault: Option<&'b solana_account_info::AccountInfo<'a>>,
    mint: Option<&'b solana_account_info::AccountInfo<'a>>,
    global: Option<&'b solana_account_info::AccountInfo<'a>>,
    bonding_curve: Option<&'b solana_account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_account_info::AccountInfo<'a>>,
    event_authority: Option<&'b solana_account_info::AccountInfo<'a>>,
    program: Option<&'b solana_account_info::AccountInfo<'a>>,
    __remaining_accounts: Vec<(&'b solana_account_info::AccountInfo<'a>, bool, bool)>,
}
