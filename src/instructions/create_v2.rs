use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_pubkey::Pubkey;

pub const CREATE_V2_DISCRIMINATOR: [u8; 8] = [214, 144, 76, 236, 95, 139, 49, 180];

#[derive(Debug)]
pub struct CreateV2 {
    pub mint: solana_pubkey::Pubkey,

    pub mint_authority: solana_pubkey::Pubkey,

    pub bonding_curve: solana_pubkey::Pubkey,

    pub associated_bonding_curve: solana_pubkey::Pubkey,

    pub global: solana_pubkey::Pubkey,

    pub user: solana_pubkey::Pubkey,

    pub system_program: solana_pubkey::Pubkey,

    pub token_program: solana_pubkey::Pubkey,

    pub associated_token_program: solana_pubkey::Pubkey,

    pub mayhem_program_id: solana_pubkey::Pubkey,

    pub global_params: solana_pubkey::Pubkey,

    pub sol_vault: solana_pubkey::Pubkey,

    pub mayhem_state: solana_pubkey::Pubkey,

    pub mayhem_token_vault: solana_pubkey::Pubkey,

    pub event_authority: solana_pubkey::Pubkey,

    pub program: solana_pubkey::Pubkey,
}

impl CreateV2 {
    pub fn instruction(&self, args: CreateV2InstructionArgs) -> solana_instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CreateV2InstructionArgs,
        remaining_accounts: &[solana_instruction::AccountMeta],
    ) -> solana_instruction::Instruction {
        let mut accounts = Vec::with_capacity(16 + remaining_accounts.len());
        accounts.push(solana_instruction::AccountMeta::new(self.mint, true));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.mint_authority,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            self.bonding_curve,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            self.associated_bonding_curve,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.global,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(self.user, true));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.associated_token_program,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            self.mayhem_program_id,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.global_params,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            self.sol_vault,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            self.mayhem_state,
            false,
        ));
        accounts.push(solana_instruction::AccountMeta::new(
            self.mayhem_token_vault,
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
        let mut data = CreateV2InstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_instruction::Instruction {
            program_id: crate::PUMP_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateV2InstructionData {
    discriminator: [u8; 8],
}

impl CreateV2InstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: CREATE_V2_DISCRIMINATOR,
        }
    }

    pub(crate) fn try_to_vec(&self) -> Result<Vec<u8>, std::io::Error> {
        borsh::to_vec(self)
    }
}

impl Default for CreateV2InstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateV2InstructionArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub creator: Pubkey,
    pub is_mayhem_mode: bool,
}

impl CreateV2InstructionArgs {
    pub(crate) fn try_to_vec(&self) -> Result<Vec<u8>, std::io::Error> {
        borsh::to_vec(self)
    }
}

#[derive(Clone, Debug, Default)]
pub struct CreateV2Builder {
    mint: Option<solana_pubkey::Pubkey>,
    mint_authority: Option<solana_pubkey::Pubkey>,
    bonding_curve: Option<solana_pubkey::Pubkey>,
    associated_bonding_curve: Option<solana_pubkey::Pubkey>,
    global: Option<solana_pubkey::Pubkey>,
    user: Option<solana_pubkey::Pubkey>,
    system_program: Option<solana_pubkey::Pubkey>,
    token_program: Option<solana_pubkey::Pubkey>,
    associated_token_program: Option<solana_pubkey::Pubkey>,
    mayhem_program_id: Option<solana_pubkey::Pubkey>,
    global_params: Option<solana_pubkey::Pubkey>,
    sol_vault: Option<solana_pubkey::Pubkey>,
    mayhem_state: Option<solana_pubkey::Pubkey>,
    mayhem_token_vault: Option<solana_pubkey::Pubkey>,
    event_authority: Option<solana_pubkey::Pubkey>,
    program: Option<solana_pubkey::Pubkey>,
    name: Option<String>,
    symbol: Option<String>,
    uri: Option<String>,
    creator: Option<Pubkey>,
    is_mayhem_mode: Option<bool>,
    __remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl CreateV2Builder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn mint(&mut self, mint: solana_pubkey::Pubkey) -> &mut Self {
        self.mint = Some(mint);
        self
    }
    #[inline(always)]
    pub fn mint_authority(&mut self, mint_authority: solana_pubkey::Pubkey) -> &mut Self {
        self.mint_authority = Some(mint_authority);
        self
    }
    #[inline(always)]
    pub fn bonding_curve(&mut self, bonding_curve: solana_pubkey::Pubkey) -> &mut Self {
        self.bonding_curve = Some(bonding_curve);
        self
    }
    #[inline(always)]
    pub fn associated_bonding_curve(
        &mut self,
        associated_bonding_curve: solana_pubkey::Pubkey,
    ) -> &mut Self {
        self.associated_bonding_curve = Some(associated_bonding_curve);
        self
    }
    #[inline(always)]
    pub fn global(&mut self, global: solana_pubkey::Pubkey) -> &mut Self {
        self.global = Some(global);
        self
    }
    #[inline(always)]
    pub fn user(&mut self, user: solana_pubkey::Pubkey) -> &mut Self {
        self.user = Some(user);
        self
    }
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn associated_token_program(
        &mut self,
        associated_token_program: solana_pubkey::Pubkey,
    ) -> &mut Self {
        self.associated_token_program = Some(associated_token_program);
        self
    }
    #[inline(always)]
    pub fn mayhem_program_id(&mut self, mayhem_program_id: solana_pubkey::Pubkey) -> &mut Self {
        self.mayhem_program_id = Some(mayhem_program_id);
        self
    }
    #[inline(always)]
    pub fn global_params(&mut self, global_params: solana_pubkey::Pubkey) -> &mut Self {
        self.global_params = Some(global_params);
        self
    }
    #[inline(always)]
    pub fn sol_vault(&mut self, sol_vault: solana_pubkey::Pubkey) -> &mut Self {
        self.sol_vault = Some(sol_vault);
        self
    }
    #[inline(always)]
    pub fn mayhem_state(&mut self, mayhem_state: solana_pubkey::Pubkey) -> &mut Self {
        self.mayhem_state = Some(mayhem_state);
        self
    }
    #[inline(always)]
    pub fn mayhem_token_vault(&mut self, mayhem_token_vault: solana_pubkey::Pubkey) -> &mut Self {
        self.mayhem_token_vault = Some(mayhem_token_vault);
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
    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }
    #[inline(always)]
    pub fn symbol(&mut self, symbol: String) -> &mut Self {
        self.symbol = Some(symbol);
        self
    }
    #[inline(always)]
    pub fn uri(&mut self, uri: String) -> &mut Self {
        self.uri = Some(uri);
        self
    }
    #[inline(always)]
    pub fn creator(&mut self, creator: Pubkey) -> &mut Self {
        self.creator = Some(creator);
        self
    }
    #[inline(always)]
    pub fn is_mayhem_mode(&mut self, is_mayhem_mode: bool) -> &mut Self {
        self.is_mayhem_mode = Some(is_mayhem_mode);
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
        let accounts = CreateV2 {
            mint: self.mint.expect("mint is not set"),
            mint_authority: self.mint_authority.expect("mint_authority is not set"),
            bonding_curve: self.bonding_curve.expect("bonding_curve is not set"),
            associated_bonding_curve: self
                .associated_bonding_curve
                .expect("associated_bonding_curve is not set"),
            global: self.global.expect("global is not set"),
            user: self.user.expect("user is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_pubkey::pubkey!("11111111111111111111111111111111")),
            token_program: self.token_program.unwrap_or(solana_pubkey::pubkey!(
                "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
            )),
            associated_token_program: self.associated_token_program.unwrap_or(
                solana_pubkey::pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
            ),
            mayhem_program_id: self.mayhem_program_id.unwrap_or(solana_pubkey::pubkey!(
                "MAyhSmzXzV1pTf7LsNkrNwkWKTo4ougAJ1PPg47MD4e"
            )),
            global_params: self.global_params.expect("global_params is not set"),
            sol_vault: self.sol_vault.expect("sol_vault is not set"),
            mayhem_state: self.mayhem_state.expect("mayhem_state is not set"),
            mayhem_token_vault: self.mayhem_token_vault.expect("mayhem_token_vault is not set"),
            event_authority: self.event_authority.expect("event_authority is not set"),
            program: self.program.expect("program is not set"),
        };
        let args = CreateV2InstructionArgs {
            name: self.name.clone().expect("name is not set"),
            symbol: self.symbol.clone().expect("symbol is not set"),
            uri: self.uri.clone().expect("uri is not set"),
            creator: self.creator.clone().expect("creator is not set"),
            is_mayhem_mode: self.is_mayhem_mode.expect("is_mayhem_mode is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}
