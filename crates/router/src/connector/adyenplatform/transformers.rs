use cards::CardNumber;
use common_utils::types::MinorUnit;
use error_stack::Report;
use masking::Secret;
use serde::Serialize;

#[cfg(feature = "payouts")]
pub mod payouts;
#[cfg(feature = "payouts")]
pub use payouts::*;

use crate::{core::errors, types};

// Error signature
type Error = Report<errors::ConnectorError>;

// Auth Struct
pub struct AdyenplatformAuthType {
    pub(super) api_key: Secret<String>,
}

impl TryFrom<&types::ConnectorAuthType> for AdyenplatformAuthType {
    type Error = Error;
    fn try_from(auth_type: &types::ConnectorAuthType) -> Result<Self, Self::Error> {
        match auth_type {
            types::ConnectorAuthType::HeaderKey { api_key } => Ok(Self {
                api_key: api_key.to_owned(),
            }),
            _ => Err(errors::ConnectorError::FailedToObtainAuthType.into()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AdyenPlatformRouterData<T> {
    pub amount: MinorUnit,
    pub router_data: T,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Amount {
    pub currency: storage_enums::Currency,
    pub value: MinorUnit,
}

impl<T> TryFrom<(MinorUnit, T)> for AdyenPlatformRouterData<T> {
    type Error = Report<errors::ConnectorError>;
    fn try_from((amount, item): (MinorUnit, T)) -> Result<Self, Self::Error> {
        Ok(Self {
            amount,
            router_data: item,
        })
    }
}

#[cfg(feature = "payouts")]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdyenPlatformPayoutEligibilityRequest {
    amount: Amount,
    merchant_account: Secret<String>,
    payment_method: PayoutCardDetails,
    reference: String,
    shopper_reference: String,
}

#[cfg(feature = "payouts")]
#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PayoutCardDetails {
    #[serde(rename = "type")]
    payment_method_type: String,
    number: CardNumber,
    expiry_month: Secret<String>,
    expiry_year: Secret<String>,
    holder_name: Secret<String>,
}

#[cfg(feature = "payouts")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdyenPayoutResponse {
    psp_reference: String,
    result_code: Option<AdyenStatus>,
    response: Option<AdyenStatus>,
    amount: Option<Amount>,
    merchant_reference: Option<String>,
    refusal_reason: Option<String>,
    refusal_reason_code: Option<String>,
    additional_data: Option<AdditionalData>,
    auth_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdyenStatus {
    AuthenticationFinished,
    AuthenticationNotRequired,
    Authorised,
    Cancelled,
    ChallengeShopper,
    Error,
    IdentifyShopper,
    PartiallyAuthorised,
    Pending,
    Received,
    RedirectShopper,
    Refused,
    PresentToShopper,
    #[cfg(feature = "payouts")]
    #[serde(rename = "[payout-confirm-received]")]
    PayoutConfirmReceived,
    #[cfg(feature = "payouts")]
    #[serde(rename = "[payout-decline-received]")]
    PayoutDeclineReceived,
    #[cfg(feature = "payouts")]
    #[serde(rename = "[payout-submit-received]")]
    PayoutSubmitReceived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AdyenRecurringModel {
    Subscription,
    UnscheduledCardOnFile,
    CardOnFile,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub enum AuthType {
    #[default]
    PreAuth,
    FinalAuth,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalData {
    authorisation_type: Option<AuthType>,
    manual_capture: Option<String>,
    execute_three_d: Option<String>,
    pub recurring_processing_model: Option<AdyenRecurringModel>,
    /// Enable recurring details in dashboard to receive this ID, https://docs.adyen.com/online-payments/tokenization/create-and-use-tokens#test-and-go-live
    #[serde(rename = "recurring.recurringDetailReference")]
    recurring_detail_reference: Option<Secret<String>>,
    #[serde(rename = "recurring.shopperReference")]
    recurring_shopper_reference: Option<String>,
    network_tx_reference: Option<Secret<String>>,
    #[cfg(feature = "payouts")]
    payout_eligible: Option<PayoutEligibility>,
    funds_availability: Option<String>,
}

pub enum PayoutEligibility {
    #[serde(rename = "Y")]
    Yes,
    #[serde(rename = "N")]
    #[default]
    No,
    #[serde(rename = "D")]
    Domestic,
    #[serde(rename = "U")]
    Unknown,
}
