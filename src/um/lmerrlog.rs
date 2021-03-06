// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
use shared::minwindef::{DWORD, LPBYTE};
use um::winnt::{LPWSTR};

STRUCT!{struct ERROR_LOG {
    el_len: DWORD,
    el_reserved: DWORD,
    el_time: DWORD,
    el_error: DWORD,
    el_name: LPWSTR,
    el_text: LPWSTR,
    el_data: LPBYTE,
    el_data_size: DWORD,
    el_nstrings: DWORD,
}}
pub type PERROR_LOG = *mut ERROR_LOG;
pub type LPERROR_LOG = *mut ERROR_LOG;
STRUCT!{struct HLOG {
    time: DWORD,
    last_flags: DWORD,
    offset: DWORD,
    rec_offset: DWORD,
}}
pub type PHLOG = *mut HLOG;
pub type LPHLOG = *mut HLOG;
pub const LOGFLAGS_FORWARD: DWORD = 0;
pub const LOGFLAGS_BACKWARD: DWORD = 0x1;
pub const LOGFLAGS_SEEK: DWORD = 0x2;
pub const ERRLOG_BASE: DWORD = 3100;
pub const NELOG_Internal_Error: DWORD = ERRLOG_BASE + 0;
pub const NELOG_Resource_Shortage: DWORD = ERRLOG_BASE + 1;
pub const NELOG_Unable_To_Lock_Segment: DWORD = ERRLOG_BASE + 2;
pub const NELOG_Unable_To_Unlock_Segment: DWORD = ERRLOG_BASE + 3;
pub const NELOG_Uninstall_Service: DWORD = ERRLOG_BASE + 4;
pub const NELOG_Init_Exec_Fail: DWORD = ERRLOG_BASE + 5;
pub const NELOG_Ncb_Error: DWORD = ERRLOG_BASE + 6;
pub const NELOG_Net_Not_Started: DWORD = ERRLOG_BASE + 7;
pub const NELOG_Ioctl_Error: DWORD = ERRLOG_BASE + 8;
pub const NELOG_System_Semaphore: DWORD = ERRLOG_BASE + 9;
pub const NELOG_Init_OpenCreate_Err: DWORD = ERRLOG_BASE + 10;
pub const NELOG_NetBios: DWORD = ERRLOG_BASE + 11;
pub const NELOG_SMB_Illegal: DWORD = ERRLOG_BASE + 12;
pub const NELOG_Service_Fail: DWORD = ERRLOG_BASE + 13;
pub const NELOG_Entries_Lost: DWORD = ERRLOG_BASE + 14;
pub const NELOG_Init_Seg_Overflow: DWORD = ERRLOG_BASE + 20;
pub const NELOG_Srv_No_Mem_Grow: DWORD = ERRLOG_BASE + 21;
pub const NELOG_Access_File_Bad: DWORD = ERRLOG_BASE + 22;
pub const NELOG_Srvnet_Not_Started: DWORD = ERRLOG_BASE + 23;
pub const NELOG_Init_Chardev_Err: DWORD = ERRLOG_BASE + 24;
pub const NELOG_Remote_API: DWORD = ERRLOG_BASE + 25;
pub const NELOG_Ncb_TooManyErr: DWORD = ERRLOG_BASE + 26;
pub const NELOG_Mailslot_err: DWORD = ERRLOG_BASE + 27;
pub const NELOG_ReleaseMem_Alert: DWORD = ERRLOG_BASE + 28;
pub const NELOG_AT_cannot_write: DWORD = ERRLOG_BASE + 29;
pub const NELOG_Cant_Make_Msg_File: DWORD = ERRLOG_BASE + 30;
pub const NELOG_Exec_Netservr_NoMem: DWORD = ERRLOG_BASE + 31;
pub const NELOG_Server_Lock_Failure: DWORD = ERRLOG_BASE + 32;
pub const NELOG_Msg_Shutdown: DWORD = ERRLOG_BASE + 40;
pub const NELOG_Msg_Sem_Shutdown: DWORD = ERRLOG_BASE + 41;
pub const NELOG_Msg_Log_Err: DWORD = ERRLOG_BASE + 50;
pub const NELOG_VIO_POPUP_ERR: DWORD = ERRLOG_BASE + 51;
pub const NELOG_Msg_Unexpected_SMB_Type: DWORD = ERRLOG_BASE + 52;
pub const NELOG_Wksta_Infoseg: DWORD = ERRLOG_BASE + 60;
pub const NELOG_Wksta_Compname: DWORD = ERRLOG_BASE + 61;
pub const NELOG_Wksta_BiosThreadFailure: DWORD = ERRLOG_BASE + 62;
pub const NELOG_Wksta_IniSeg: DWORD = ERRLOG_BASE + 63;
pub const NELOG_Wksta_HostTab_Full: DWORD = ERRLOG_BASE + 64;
pub const NELOG_Wksta_Bad_Mailslot_SMB: DWORD = ERRLOG_BASE + 65;
pub const NELOG_Wksta_UASInit: DWORD = ERRLOG_BASE + 66;
pub const NELOG_Wksta_SSIRelogon: DWORD = ERRLOG_BASE + 67;
pub const NELOG_Build_Name: DWORD = ERRLOG_BASE + 70;
pub const NELOG_Name_Expansion: DWORD = ERRLOG_BASE + 71;
pub const NELOG_Message_Send: DWORD = ERRLOG_BASE + 72;
pub const NELOG_Mail_Slt_Err: DWORD = ERRLOG_BASE + 73;
pub const NELOG_AT_cannot_read: DWORD = ERRLOG_BASE + 74;
pub const NELOG_AT_sched_err: DWORD = ERRLOG_BASE + 75;
pub const NELOG_AT_schedule_file_created: DWORD = ERRLOG_BASE + 76;
pub const NELOG_Srvnet_NB_Open: DWORD = ERRLOG_BASE + 77;
pub const NELOG_AT_Exec_Err: DWORD = ERRLOG_BASE + 78;
pub const NELOG_Lazy_Write_Err: DWORD = ERRLOG_BASE + 80;
pub const NELOG_HotFix: DWORD = ERRLOG_BASE + 81;
pub const NELOG_HardErr_From_Server: DWORD = ERRLOG_BASE + 82;
pub const NELOG_LocalSecFail1: DWORD = ERRLOG_BASE + 83;
pub const NELOG_LocalSecFail2: DWORD = ERRLOG_BASE + 84;
pub const NELOG_LocalSecFail3: DWORD = ERRLOG_BASE + 85;
pub const NELOG_LocalSecGeneralFail: DWORD = ERRLOG_BASE + 86;
pub const NELOG_NetWkSta_Internal_Error: DWORD = ERRLOG_BASE + 90;
pub const NELOG_NetWkSta_No_Resource: DWORD = ERRLOG_BASE + 91;
pub const NELOG_NetWkSta_SMB_Err: DWORD = ERRLOG_BASE + 92;
pub const NELOG_NetWkSta_VC_Err: DWORD = ERRLOG_BASE + 93;
pub const NELOG_NetWkSta_Stuck_VC_Err: DWORD = ERRLOG_BASE + 94;
pub const NELOG_NetWkSta_NCB_Err: DWORD = ERRLOG_BASE + 95;
pub const NELOG_NetWkSta_Write_Behind_Err: DWORD = ERRLOG_BASE + 96;
pub const NELOG_NetWkSta_Reset_Err: DWORD = ERRLOG_BASE + 97;
pub const NELOG_NetWkSta_Too_Many: DWORD = ERRLOG_BASE + 98;
pub const NELOG_Srv_Thread_Failure: DWORD = ERRLOG_BASE + 104;
pub const NELOG_Srv_Close_Failure: DWORD = ERRLOG_BASE + 105;
pub const NELOG_ReplUserCurDir: DWORD = ERRLOG_BASE + 106;
pub const NELOG_ReplCannotMasterDir: DWORD = ERRLOG_BASE + 107;
pub const NELOG_ReplUpdateError: DWORD = ERRLOG_BASE + 108;
pub const NELOG_ReplLostMaster: DWORD = ERRLOG_BASE + 109;
pub const NELOG_NetlogonAuthDCFail: DWORD = ERRLOG_BASE + 110;
pub const NELOG_ReplLogonFailed: DWORD = ERRLOG_BASE + 111;
pub const NELOG_ReplNetErr: DWORD = ERRLOG_BASE + 112;
pub const NELOG_ReplMaxFiles: DWORD = ERRLOG_BASE + 113;
pub const NELOG_ReplMaxTreeDepth: DWORD = ERRLOG_BASE + 114;
pub const NELOG_ReplBadMsg: DWORD = ERRLOG_BASE + 115;
pub const NELOG_ReplSysErr: DWORD = ERRLOG_BASE + 116;
pub const NELOG_ReplUserLoged: DWORD = ERRLOG_BASE + 117;
pub const NELOG_ReplBadImport: DWORD = ERRLOG_BASE + 118;
pub const NELOG_ReplBadExport: DWORD = ERRLOG_BASE + 119;
pub const NELOG_ReplSignalFileErr: DWORD = ERRLOG_BASE + 120;
pub const NELOG_DiskFT: DWORD = ERRLOG_BASE + 121;
pub const NELOG_ReplAccessDenied: DWORD = ERRLOG_BASE + 122;
pub const NELOG_NetlogonFailedPrimary: DWORD = ERRLOG_BASE + 123;
pub const NELOG_NetlogonPasswdSetFailed: DWORD = ERRLOG_BASE + 124;
pub const NELOG_NetlogonTrackingError: DWORD = ERRLOG_BASE + 125;
pub const NELOG_NetlogonSyncError: DWORD = ERRLOG_BASE + 126;
pub const NELOG_NetlogonRequireSignOrSealError: DWORD = ERRLOG_BASE + 127;
pub const NELOG_UPS_PowerOut: DWORD = ERRLOG_BASE + 130;
pub const NELOG_UPS_Shutdown: DWORD = ERRLOG_BASE + 131;
pub const NELOG_UPS_CmdFileError: DWORD = ERRLOG_BASE + 132;
pub const NELOG_UPS_CannotOpenDriver: DWORD = ERRLOG_BASE+133;
pub const NELOG_UPS_PowerBack: DWORD = ERRLOG_BASE + 134;
pub const NELOG_UPS_CmdFileConfig: DWORD = ERRLOG_BASE + 135;
pub const NELOG_UPS_CmdFileExec: DWORD = ERRLOG_BASE + 136;
pub const NELOG_Missing_Parameter: DWORD = ERRLOG_BASE + 150;
pub const NELOG_Invalid_Config_Line: DWORD = ERRLOG_BASE + 151;
pub const NELOG_Invalid_Config_File: DWORD = ERRLOG_BASE + 152;
pub const NELOG_File_Changed: DWORD = ERRLOG_BASE + 153;
pub const NELOG_Files_Dont_Fit: DWORD = ERRLOG_BASE + 154;
pub const NELOG_Wrong_DLL_Version: DWORD = ERRLOG_BASE + 155;
pub const NELOG_Error_in_DLL: DWORD = ERRLOG_BASE + 156;
pub const NELOG_System_Error: DWORD = ERRLOG_BASE + 157;
pub const NELOG_FT_ErrLog_Too_Large: DWORD = ERRLOG_BASE + 158;
pub const NELOG_FT_Update_In_Progress: DWORD = ERRLOG_BASE + 159;
pub const NELOG_Joined_Domain: DWORD = ERRLOG_BASE + 160;
pub const NELOG_Joined_Workgroup: DWORD = ERRLOG_BASE + 161;
pub const NELOG_OEM_Code: DWORD = ERRLOG_BASE + 199;
pub const ERRLOG2_BASE: DWORD = 5700;
pub const NELOG_NetlogonSSIInitError: DWORD = ERRLOG2_BASE + 0;
pub const NELOG_NetlogonFailedToUpdateTrustList: DWORD = ERRLOG2_BASE + 1;
pub const NELOG_NetlogonFailedToAddRpcInterface: DWORD = ERRLOG2_BASE + 2;
pub const NELOG_NetlogonFailedToReadMailslot: DWORD = ERRLOG2_BASE + 3;
pub const NELOG_NetlogonFailedToRegisterSC: DWORD = ERRLOG2_BASE + 4;
pub const NELOG_NetlogonChangeLogCorrupt: DWORD = ERRLOG2_BASE + 5;
pub const NELOG_NetlogonFailedToCreateShare: DWORD = ERRLOG2_BASE + 6;
pub const NELOG_NetlogonDownLevelLogonFailed: DWORD = ERRLOG2_BASE + 7;
pub const NELOG_NetlogonDownLevelLogoffFailed: DWORD = ERRLOG2_BASE + 8;
pub const NELOG_NetlogonNTLogonFailed: DWORD = ERRLOG2_BASE + 9;
pub const NELOG_NetlogonNTLogoffFailed: DWORD = ERRLOG2_BASE + 10;
pub const NELOG_NetlogonPartialSyncCallSuccess: DWORD = ERRLOG2_BASE + 11;
pub const NELOG_NetlogonPartialSyncCallFailed: DWORD = ERRLOG2_BASE + 12;
pub const NELOG_NetlogonFullSyncCallSuccess: DWORD = ERRLOG2_BASE + 13;
pub const NELOG_NetlogonFullSyncCallFailed: DWORD = ERRLOG2_BASE + 14;
pub const NELOG_NetlogonPartialSyncSuccess: DWORD = ERRLOG2_BASE + 15;
pub const NELOG_NetlogonPartialSyncFailed: DWORD = ERRLOG2_BASE + 16;
pub const NELOG_NetlogonFullSyncSuccess: DWORD = ERRLOG2_BASE + 17;
pub const NELOG_NetlogonFullSyncFailed: DWORD = ERRLOG2_BASE + 18;
pub const NELOG_NetlogonAuthNoDomainController: DWORD = ERRLOG2_BASE + 19;
pub const NELOG_NetlogonAuthNoTrustLsaSecret: DWORD = ERRLOG2_BASE + 20;
pub const NELOG_NetlogonAuthNoTrustSamAccount: DWORD = ERRLOG2_BASE + 21;
pub const NELOG_NetlogonServerAuthFailed: DWORD = ERRLOG2_BASE + 22;
pub const NELOG_NetlogonServerAuthNoTrustSamAccount: DWORD = ERRLOG2_BASE + 23;
pub const NELOG_FailedToRegisterSC: DWORD = ERRLOG2_BASE + 24;
pub const NELOG_FailedToSetServiceStatus: DWORD = ERRLOG2_BASE + 25;
pub const NELOG_FailedToGetComputerName: DWORD = ERRLOG2_BASE + 26;
pub const NELOG_DriverNotLoaded: DWORD = ERRLOG2_BASE + 27;
pub const NELOG_NoTranportLoaded: DWORD = ERRLOG2_BASE + 28;
pub const NELOG_NetlogonFailedDomainDelta: DWORD = ERRLOG2_BASE + 29;
pub const NELOG_NetlogonFailedGlobalGroupDelta: DWORD = ERRLOG2_BASE + 30;
pub const NELOG_NetlogonFailedLocalGroupDelta: DWORD = ERRLOG2_BASE + 31;
pub const NELOG_NetlogonFailedUserDelta: DWORD = ERRLOG2_BASE + 32;
pub const NELOG_NetlogonFailedPolicyDelta: DWORD = ERRLOG2_BASE + 33;
pub const NELOG_NetlogonFailedTrustedDomainDelta: DWORD = ERRLOG2_BASE + 34;
pub const NELOG_NetlogonFailedAccountDelta: DWORD = ERRLOG2_BASE + 35;
pub const NELOG_NetlogonFailedSecretDelta: DWORD = ERRLOG2_BASE + 36;
pub const NELOG_NetlogonSystemError: DWORD = ERRLOG2_BASE + 37;
pub const NELOG_NetlogonDuplicateMachineAccounts: DWORD = ERRLOG2_BASE + 38;
pub const NELOG_NetlogonTooManyGlobalGroups: DWORD = ERRLOG2_BASE + 39;
pub const NELOG_NetlogonBrowserDriver: DWORD = ERRLOG2_BASE + 40;
pub const NELOG_NetlogonAddNameFailure: DWORD = ERRLOG2_BASE + 41;
pub const NELOG_RplMessages: DWORD = ERRLOG2_BASE + 42;
pub const NELOG_RplXnsBoot: DWORD = ERRLOG2_BASE + 43;
pub const NELOG_RplSystem: DWORD = ERRLOG2_BASE + 44;
pub const NELOG_RplWkstaTimeout: DWORD = ERRLOG2_BASE + 45;
pub const NELOG_RplWkstaFileOpen: DWORD = ERRLOG2_BASE + 46;
pub const NELOG_RplWkstaFileRead: DWORD = ERRLOG2_BASE + 47;
pub const NELOG_RplWkstaMemory: DWORD = ERRLOG2_BASE + 48;
pub const NELOG_RplWkstaFileChecksum: DWORD = ERRLOG2_BASE + 49;
pub const NELOG_RplWkstaFileLineCount: DWORD = ERRLOG2_BASE + 50;
pub const NELOG_RplWkstaBbcFile: DWORD = ERRLOG2_BASE + 51;
pub const NELOG_RplWkstaFileSize: DWORD = ERRLOG2_BASE + 52;
pub const NELOG_RplWkstaInternal: DWORD = ERRLOG2_BASE + 53;
pub const NELOG_RplWkstaWrongVersion: DWORD = ERRLOG2_BASE + 54;
pub const NELOG_RplWkstaNetwork: DWORD = ERRLOG2_BASE + 55;
pub const NELOG_RplAdapterResource: DWORD = ERRLOG2_BASE + 56;
pub const NELOG_RplFileCopy: DWORD = ERRLOG2_BASE + 57;
pub const NELOG_RplFileDelete: DWORD = ERRLOG2_BASE + 58;
pub const NELOG_RplFilePerms: DWORD = ERRLOG2_BASE + 59;
pub const NELOG_RplCheckConfigs: DWORD = ERRLOG2_BASE + 60;
pub const NELOG_RplCreateProfiles: DWORD = ERRLOG2_BASE + 61;
pub const NELOG_RplRegistry: DWORD = ERRLOG2_BASE + 62;
pub const NELOG_RplReplaceRPLDISK: DWORD = ERRLOG2_BASE + 63;
pub const NELOG_RplCheckSecurity: DWORD = ERRLOG2_BASE + 64;
pub const NELOG_RplBackupDatabase: DWORD = ERRLOG2_BASE + 65;
pub const NELOG_RplInitDatabase: DWORD = ERRLOG2_BASE + 66;
pub const NELOG_RplRestoreDatabaseFailure: DWORD = ERRLOG2_BASE + 67;
pub const NELOG_RplRestoreDatabaseSuccess: DWORD = ERRLOG2_BASE + 68;
pub const NELOG_RplInitRestoredDatabase: DWORD = ERRLOG2_BASE + 69;
pub const NELOG_NetlogonSessionTypeWrong: DWORD = ERRLOG2_BASE + 70;
pub const NELOG_RplUpgradeDBTo40: DWORD = ERRLOG2_BASE + 71;
pub const NELOG_NetlogonLanmanBdcsNotAllowed: DWORD = ERRLOG2_BASE + 72;
pub const NELOG_NetlogonNoDynamicDns: DWORD = ERRLOG2_BASE + 73;
pub const NELOG_NetlogonDynamicDnsRegisterFailure: DWORD = ERRLOG2_BASE + 74;
pub const NELOG_NetlogonDynamicDnsDeregisterFailure: DWORD = ERRLOG2_BASE + 75;
pub const NELOG_NetlogonFailedFileCreate: DWORD = ERRLOG2_BASE + 76;
pub const NELOG_NetlogonGetSubnetToSite: DWORD = ERRLOG2_BASE + 77;
pub const NELOG_NetlogonNoSiteForClient: DWORD = ERRLOG2_BASE + 78;
pub const NELOG_NetlogonBadSiteName: DWORD = ERRLOG2_BASE + 79;
pub const NELOG_NetlogonBadSubnetName: DWORD = ERRLOG2_BASE + 80;
pub const NELOG_NetlogonDynamicDnsServerFailure: DWORD = ERRLOG2_BASE + 81;
pub const NELOG_NetlogonDynamicDnsFailure: DWORD = ERRLOG2_BASE + 82;
pub const NELOG_NetlogonRpcCallCancelled: DWORD = ERRLOG2_BASE + 83;
pub const NELOG_NetlogonDcSiteCovered: DWORD = ERRLOG2_BASE + 84;
pub const NELOG_NetlogonDcSiteNotCovered: DWORD = ERRLOG2_BASE + 85;
pub const NELOG_NetlogonGcSiteCovered: DWORD = ERRLOG2_BASE + 86;
pub const NELOG_NetlogonGcSiteNotCovered: DWORD = ERRLOG2_BASE + 87;
pub const NELOG_NetlogonFailedSpnUpdate: DWORD = ERRLOG2_BASE + 88;
pub const NELOG_NetlogonFailedDnsHostNameUpdate: DWORD = ERRLOG2_BASE + 89;
pub const NELOG_NetlogonAuthNoUplevelDomainController: DWORD = ERRLOG2_BASE + 90;
pub const NELOG_NetlogonAuthDomainDowngraded: DWORD = ERRLOG2_BASE + 91;
pub const NELOG_NetlogonNdncSiteCovered: DWORD = ERRLOG2_BASE + 92;
pub const NELOG_NetlogonNdncSiteNotCovered: DWORD = ERRLOG2_BASE + 93;
pub const NELOG_NetlogonDcOldSiteCovered: DWORD = ERRLOG2_BASE + 94;
pub const NELOG_NetlogonDcSiteNotCoveredAuto: DWORD = ERRLOG2_BASE + 95;
pub const NELOG_NetlogonGcOldSiteCovered: DWORD = ERRLOG2_BASE + 96;
pub const NELOG_NetlogonGcSiteNotCoveredAuto: DWORD = ERRLOG2_BASE + 97;
pub const NELOG_NetlogonNdncOldSiteCovered: DWORD = ERRLOG2_BASE + 98;
pub const NELOG_NetlogonNdncSiteNotCoveredAuto: DWORD = ERRLOG2_BASE + 99;
pub const NELOG_NetlogonSpnMultipleSamAccountNames: DWORD = ERRLOG2_BASE + 100;
pub const NELOG_NetlogonSpnCrackNamesFailure: DWORD = ERRLOG2_BASE + 101;
pub const NELOG_NetlogonNoAddressToSiteMapping: DWORD = ERRLOG2_BASE + 102;
pub const NELOG_NetlogonInvalidGenericParameterValue: DWORD = ERRLOG2_BASE + 103;
pub const NELOG_NetlogonInvalidDwordParameterValue: DWORD = ERRLOG2_BASE + 104;
pub const NELOG_NetlogonServerAuthFailedNoAccount: DWORD = ERRLOG2_BASE + 105;
pub const NELOG_NetlogonNoDynamicDnsManual: DWORD = ERRLOG2_BASE + 106;
pub const NELOG_NetlogonNoSiteForClients: DWORD = ERRLOG2_BASE + 107;
pub const NELOG_NetlogonDnsDeregAborted: DWORD = ERRLOG2_BASE + 108;
pub const NELOG_NetlogonRpcPortRequestFailure: DWORD = ERRLOG2_BASE + 109;
pub const NELOG_NetlogonPartialSiteMappingForClients: DWORD = ERRLOG2_BASE + 110;
pub const NELOG_NetlogonRemoteDynamicDnsRegisterFailure: DWORD = ERRLOG2_BASE + 111;
pub const NELOG_NetlogonRemoteDynamicDnsDeregisterFailure: DWORD = ERRLOG2_BASE + 112;
pub const NELOG_NetlogonRejectedRemoteDynamicDnsRegister: DWORD = ERRLOG2_BASE + 113;
pub const NELOG_NetlogonRejectedRemoteDynamicDnsDeregister: DWORD = ERRLOG2_BASE + 114;
pub const NELOG_NetlogonRemoteDynamicDnsUpdateRequestFailure: DWORD = ERRLOG2_BASE + 115;
pub const NELOG_NetlogonUserValidationReqInitialTimeOut: DWORD = ERRLOG2_BASE + 116;
pub const NELOG_NetlogonUserValidationReqRecurringTimeOut: DWORD = ERRLOG2_BASE + 117;
pub const NELOG_NetlogonUserValidationReqWaitInitialWarning: DWORD = ERRLOG2_BASE + 118;
pub const NELOG_NetlogonUserValidationReqWaitRecurringWarning: DWORD = ERRLOG2_BASE + 119;
pub const NELOG_NetlogonFailedToAddAuthzRpcInterface: DWORD = ERRLOG2_BASE + 120;
pub const NELOG_NetLogonFailedToInitializeAuthzRm: DWORD = ERRLOG2_BASE + 121;
pub const NELOG_NetLogonFailedToInitializeRPCSD: DWORD = ERRLOG2_BASE + 122;
pub const NELOG_NetlogonMachinePasswdSetSucceeded: DWORD = ERRLOG2_BASE + 123;
pub const NELOG_NetlogonMsaPasswdSetSucceeded: DWORD = ERRLOG2_BASE + 124;
