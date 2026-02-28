use borsh::BorshDeserialize;
use borsh::BorshSerialize;

pub const MIGRATE_BONDING_CURVE_CREATOR_DISCRIMINATOR: [u8; 8] =
    [87, 124, 52, 191, 52, 38, 214, 232];

#[derive(Debug)]
pub struct MigrateBondingCurveCreator {
    pub mint: solana_pubkey::Pubkey,

    pub bonding_curve: solana_pubkey::Pubkey,

    pub sharing_config: solana_pubkey::Pubkey,

    pub event_authority: solana_pubkey::Pubkey,

    pub program: solana_pubkey::Pubkey,
}

impl MigrateBondingCurveCreator {
    pub fn instruction(&self) -> solana_instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_instruction::AccountMeta],
    ) -> solana_instruction::Instruction {
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.mint,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            self.bonding_curve,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.sharing_config,
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
        let data = MigrateBondingCurveCreatorInstructionData::new()
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
pub struct MigrateBondingCurveCreatorInstructionData {
    discriminator: [u8; 8],
}

impl MigrateBondingCurveCreatorInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [87, 124, 52, 191, 52, 38, 214, 232],
        }
    }

    pub(crate) fn try_to_vec(&self) -> Result<Vec<u8>, std::io::Error> {
        borsh::to_vec(self)
    }
}

impl Default for MigrateBondingCurveCreatorInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Default)]
pub struct MigrateBondingCurveCreatorBuilder {
    mint: Option<solana_pubkey::Pubkey>,
    bonding_curve: Option<solana_pubkey::Pubkey>,
    sharing_config: Option<solana_pubkey::Pubkey>,
    event_authority: Option<solana_pubkey::Pubkey>,
    program: Option<solana_pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl MigrateBondingCurveCreatorBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn mint(&mut self, mint: solana_pubkey::Pubkey) -> &mut Self {
        self.mint = Some(mint);
        self
    }
    #[inline(always)]
    pub fn bonding_curve(&mut self, bonding_curve: solana_pubkey::Pubkey) -> &mut Self {
        self.bonding_curve = Some(bonding_curve);
        self
    }
    #[inline(always)]
    pub fn sharing_config(&mut self, sharing_config: solana_pubkey::Pubkey) -> &mut Self {
        self.sharing_config = Some(sharing_config);
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
        let accounts = MigrateBondingCurveCreator {
            mint: self.mint.expect("mint is not set"),
            bonding_curve: self.bonding_curve.expect("bonding_curve is not set"),
            sharing_config: self.sharing_config.expect("sharing_config is not set"),
            event_authority: self.event_authority.expect("event_authority is not set"),
            program: self.program.expect("program is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

pub struct MigrateBondingCurveCreatorCpiAccounts<'a, 'b> {
    pub mint: &'b solana_account_info::AccountInfo<'a>,

    pub bonding_curve: &'b solana_account_info::AccountInfo<'a>,

    pub sharing_config: &'b solana_account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_account_info::AccountInfo<'a>,

    pub program: &'b solana_account_info::AccountInfo<'a>,
}

pub struct MigrateBondingCurveCreatorCpi<'a, 'b> {
    pub __program: &'b solana_account_info::AccountInfo<'a>,

    pub mint: &'b solana_account_info::AccountInfo<'a>,

    pub bonding_curve: &'b solana_account_info::AccountInfo<'a>,

    pub sharing_config: &'b solana_account_info::AccountInfo<'a>,

    pub event_authority: &'b solana_account_info::AccountInfo<'a>,

    pub program: &'b solana_account_info::AccountInfo<'a>,
}

impl<'a, 'b> MigrateBondingCurveCreatorCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_account_info::AccountInfo<'a>,
        accounts: MigrateBondingCurveCreatorCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            mint: accounts.mint,
            bonding_curve: accounts.bonding_curve,
            sharing_config: accounts.sharing_config,
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
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.mint.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            *self.bonding_curve.key,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.sharing_config.key,
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
        let data = MigrateBondingCurveCreatorInstructionData::new()
            .try_to_vec()
            .unwrap();

        let instruction = solana_instruction::Instruction {
            program_id: crate::PUMP_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(6 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.mint.clone());
        account_infos.push(self.bonding_curve.clone());
        account_infos.push(self.sharing_config.clone());
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
pub struct MigrateBondingCurveCreatorCpiBuilder<'a, 'b> {
    instruction: Box<MigrateBondingCurveCreatorCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> MigrateBondingCurveCreatorCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(MigrateBondingCurveCreatorCpiBuilderInstruction {
            __program: program,
            mint: None,
            bonding_curve: None,
            sharing_config: None,
            event_authority: None,
            program: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn mint(&mut self, mint: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.mint = Some(mint);
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
    pub fn sharing_config(
        &mut self,
        sharing_config: &'b solana_account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.sharing_config = Some(sharing_config);
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
        let instruction = MigrateBondingCurveCreatorCpi {
            __program: self.instruction.__program,

            mint: self.instruction.mint.expect("mint is not set"),

            bonding_curve: self
                .instruction
                .bonding_curve
                .expect("bonding_curve is not set"),

            sharing_config: self
                .instruction
                .sharing_config
                .expect("sharing_config is not set"),

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
struct MigrateBondingCurveCreatorCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_account_info::AccountInfo<'a>,
    mint: Option<&'b solana_account_info::AccountInfo<'a>>,
    bonding_curve: Option<&'b solana_account_info::AccountInfo<'a>>,
    sharing_config: Option<&'b solana_account_info::AccountInfo<'a>>,
    event_authority: Option<&'b solana_account_info::AccountInfo<'a>>,
    program: Option<&'b solana_account_info::AccountInfo<'a>>,
    __remaining_accounts: Vec<(&'b solana_account_info::AccountInfo<'a>, bool, bool)>,
}
