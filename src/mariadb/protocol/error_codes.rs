use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, PartialEq, TryFromPrimitive)]
#[TryFromPrimitiveType = "i16"]
pub enum ErrorCode {
    ErDefault = 0,
    ErHashchk = 1000,
    ErNisamchk = 1001,
    ErNo = 1002,
    ErYes = 1003,
    ErCantCreateFile = 1004,
    ErCantCreateTable = 1005,
    ErCantCreateDb = 1006,
    ErDbCreateExists = 1007,
    ErDbDropExists = 1008,
    ErDbDropDelete = 1009,
    ErDbDropRmdir = 1010,
    ErCantDeleteFile = 1011,
    ErCantFindSystemRec = 1012,
    ErCantGetStat = 1013,
    ErCantGetWd = 1014,
    ErCantLock = 1015,
    ErCantOpenFile = 1016,
    ErFileNotFound = 1017,
    ErCantReadDir = 1018,
    ErCantSetWd = 1019,
    ErCheckread = 1020,
    ErDiskFull = 1021,
    ErDupKey = 1022,
    ErErrorOnClose = 1023,
    ErErrorOnRead = 1024,
    ErErrorOnRename = 1025,
    ErErrorOnWrite = 1026,
    ErFileUsed = 1027,
    ErFilsortAbort = 1028,
    ErFormNotFound = 1029,
    ErGetErrn = 1030,
    ErIllegalHa = 1031,
    ErKeyNotFound = 1032,
    ErNotFormFile = 1033,
    ErNotKeyfile = 1034,
    ErOldKeyfile = 1035,
    ErOpenAsReadonly = 1036,
    ErOutofmemory = 1037,
    ErOutOfSortmemory = 1038,
    ErUnexpectedEof = 1039,
    ErConCountError = 1040,
    ErOutOfResources = 1041,
    ErBadHostError = 1042,
    ErHandshakeError = 1043,
    ErDbaccessDeniedError = 1044,
    ErAccessDeniedError = 1045,
    ErNoDbError = 1046,
    ErUnknownComError = 1047,
    ErBadNullError = 1048,
    ErBadDbError = 1049,
    ErTableExistsError = 1050,
    ErBadTableError = 1051,
    ErNonUniqError = 1052,
    ErServerShutdown = 1053,
    ErBadFieldError = 1054,
    ErWrongFieldWithGroup = 1055,
    ErWrongGroupField = 1056,
    ErWrongSumSelect = 1057,
    ErWrongValueCount = 1058,
    ErTooLongIdent = 1059,
    ErDupFieldname = 1060,
    ErDupKeyname = 1061,
    ErDupEntry = 1062,
    ErWrongFieldSpec = 1063,
    ErParseError = 1064,
    ErEmptyQuery = 1065,
    ErNonuniqTable = 1066,
    ErInvalidDefault = 1067,
    ErMultiplePriKey = 1068,
    ErTooManyKeys = 1069,
    ErTooManyKeyParts = 1070,
    ErTooLongKey = 1071,
    ErKeyColumnDoesNotExits = 1072,
    ErBlobUsedAsKey = 1073,
    ErTooBigFieldlength = 1074,
    ErWrongAutoKey = 1075,
    ErReady = 1076,
    ErNormalShutdown = 1077,
    ErGotSignal = 1078,
    ErShutdownComplete = 1079,
    ErForcingClose = 1080,
    ErIpsockError = 1081,
    ErNoSuchIndex = 1082,
    ErWrongFieldTerminators = 1083,
    ErBlobsAndNoTerminated = 1084,
    ErTextfileNotReadable = 1085,
    ErFileExistsError = 1086,
    ErLoadInf = 1087,
    ErAlterInf = 1088,
    ErWrongSubKey = 1089,
    ErCantRemoveAllFields = 1090,
    ErCantDropFieldOrKey = 1091,
    ErInsertInf = 1092,
    ErUpdateTableUsed = 1093,
    ErNoSuchThread = 1094,
    ErKillDeniedError = 1095,
    ErNoTablesUsed = 1096,
    ErTooBigSet = 1097,
    ErNoUniqueLogfile = 1098,
    ErTableNotLockedForWrite = 1099,
    ErTableNotLocked = 1100,
    ErBlobCantHaveDefault = 1101,
    ErWrongDbName = 1102,
    ErWrongTableName = 1103,
    ErTooBigSelect = 1104,
    ErUnknownError = 1105,
    ErUnknownProcedure = 1106,
    ErWrongParamcountToProcedure = 1107,
    ErWrongParametersToProcedure = 1108,
    ErUnknownTable = 1109,
    ErFieldSpecifiedTwice = 1110,
    ErInvalidGroupFuncUse = 1111,
    ErUnsupportedExtension = 1112,
    ErTableMustHaveColumns = 1113,
    ErRecordFileFull = 1114,
    ErUnknownCharacterSet = 1115,
    ErTooManyTables = 1116,
    ErTooManyFields = 1117,
    ErTooBigRowsize = 1118,
    ErStackOverrun = 1119,
    ErWrongOuterJoin = 1120,
    ErNullColumnInIndex = 1121,
    ErCantFindUdf = 1122,
    ErCantInitializeUdf = 1123,
    ErUdfNoPaths = 1124,
    ErUdfExists = 1125,
    ErCantOpenLibrary = 1126,
    ErCantFindDlEntry = 1127,
    ErFunctionNotDefined = 1128,
    ErHostIsBlocked = 1129,
    ErHostNotPrivileged = 1130,
    ErPasswordAnonymousUser = 1131,
    ErPasswordNotAllowed = 1132,
    ErPasswordNoMatch = 1133,
    ErUpdateInf = 1134,
    ErCantCreateThread = 1135,
    ErWrongValueCountOnRow = 1136,
    ErCantReopenTable = 1137,
    ErInvalidUseOfNull = 1138,
    ErRegexpError = 1139,
    ErMixOfGroupFuncAndFields = 1140,
    ErNonexistingGrant = 1141,
    ErTableaccessDeniedError = 1142,
    ErColumnaccessDeniedError = 1143,
    ErIllegalGrantForTable = 1144,
    ErGrantWrongHostOrUser = 1145,
    ErNoSuchTable = 1146,
    ErNonexistingTableGrant = 1147,
    ErNotAllowedCommand = 1148,
    ErSyntaxError = 1149,
    ErDelayedCantChangeLock = 1150,
    ErTooManyDelayedThreads = 1151,
    ErAbortingConnection = 1152,
    ErNetPacketTooLarge = 1153,
    ErNetReadErrorFromPipe = 1154,
    ErNetFcntlError = 1155,
    ErNetPacketsOutOfOrder = 1156,
    ErNetUncompressError = 1157,
    ErNetReadError = 1158,
    ErNetReadInterrupted = 1159,
    ErNetErrorOnWrite = 1160,
    ErNetWriteInterrupted = 1161,
    ErTooLongString = 1162,
    ErTableCantHandleBlob = 1163,
    ErTableCantHandleAutoIncrement = 1164,
    ErDelayedInsertTableLocked = 1165,
    ErWrongColumnName = 1166,
    ErWrongKeyColumn = 1167,
    ErWrongMrgTable = 1168,
    ErDupUnique = 1169,
    ErBlobKeyWithoutLength = 1170,
    ErPrimaryCantHaveNull = 1171,
    ErTooManyRows = 1172,
    ErRequiresPrimaryKey = 1173,
    ErNoRaidCompiled = 1174,
    ErUpdateWithoutKeyInSafeMode = 1175,
    ErKeyDoesNotExits = 1176,
    ErCheckNoSuchTable = 1177,
    ErCheckNotImplemented = 1178,
    ErCantDoThisDuringAnTransaction = 1179,
    ErErrorDuringCommit = 1180,
    ErErrorDuringRollback = 1181,
    ErErrorDuringFlushLogs = 1182,
    ErErrorDuringCheckpoint = 1183,
    ErNewAbortingConnection = 1184,
    ErDumpNotImplemented = 1185,
    ErFlushMasterBinlogClosed = 1186,
    ErIndexRebuild = 1187,
    ErMaster = 1188,
    ErMasterNetRead = 1189,
    ErMasterNetWrite = 1190,
    ErFtMatchingKeyNotFound = 1191,
    ErLockOrActiveTransaction = 1192,
    ErUnknownSystemVariable = 1193,
    ErCrashedOnUsage = 1194,
    ErCrashedOnRepair = 1195,
    ErWarningNotCompleteRollback = 1196,
    ErTransCacheFull = 1197,
    ErSlaveMustStop = 1198,
    ErSlaveNotRunning = 1199,
    ErBadSlave = 1200,
    ErMasterInf = 1201,
    ErSlaveThread = 1202,
    ErTooManyUserConnections = 1203,
    ErSetConstantsOnly = 1204,
    ErLockWaitTimeout = 1205,
    ErLockTableFull = 1206,
    ErReadOnlyTransaction = 1207,
    ErDropDbWithReadLock = 1208,
    ErCreateDbWithReadLock = 1209,
    ErWrongArguments = 1210,
    ErNoPermissionToCreateUser = 1211,
    ErUnionTablesInDifferentDir = 1212,
    ErLockDeadlock = 1213,
    ErTableCantHandleFt = 1214,
    ErCannotAddForeign = 1215,
    ErNoReferencedRow = 1216,
    ErRowIsReferenced = 1217,
    ErConnectToMaster = 1218,
    ErQueryOnMaster = 1219,
    ErErrorWhenExecutingCommand = 1220,
    ErWrongUsage = 1221,
    ErWrongNumberOfColumnsInSelect = 1222,
    ErCantUpdateWithReadlock = 1223,
    ErMixingNotAllowed = 1224,
    ErDupArgument = 1225,
    ErUserLimitReached = 1226,
    ErSpecificAccessDeniedError = 1227,
    ErLocalVariable = 1228,
    ErGlobalVariable = 1229,
    ErNoDefault = 1230,
    ErWrongValueForVar = 1231,
    ErWrongTypeForVar = 1232,
    ErVarCantBeRead = 1233,
    ErCantUseOptionHere = 1234,
    ErNotSupportedYet = 1235,
    ErMasterFatalErrorReadingBinlog = 1236,
    ErSlaveIgnoredTable = 1237,
    ErIncorrectGlobalLocalVar = 1238,
    ErWrongFkDef = 1239,
    ErKeyRefDoNotMatchTableRef = 1240,
    ErOperandColumns = 1241,
    ErSubqueryNo1Row = 1242,
    ErUnknownStmtHandler = 1243,
    ErCorruptHelpDb = 1244,
    ErCyclicReference = 1245,
    ErAutoConvert = 1246,
    ErIllegalReference = 1247,
    ErDerivedMustHaveAlias = 1248,
    ErSelectReduced = 1249,
    ErTablenameNotAllowedHere = 1250,
    ErNotSupportedAuthMode = 1251,
    ErSpatialCantHaveNull = 1252,
    ErCollationCharsetMismatch = 1253,
    ErSlaveWasRunning = 1254,
    ErSlaveWasNotRunning = 1255,
    ErTooBigForUncompress = 1256,
    ErZlibZMemError = 1257,
    ErZlibZBufError = 1258,
    ErZlibZDataError = 1259,
    ErCutValueGroupConcat = 1260,
    ErWarnTooFewRecords = 1261,
    ErWarnTooManyRecords = 1262,
    ErWarnNullToNotnull = 1263,
    ErWarnDataOutOfRange = 1264,
    WarnDataTruncated = 1265,
    ErWarnUsingOtherHandler = 1266,
    ErCantAggregate2Collations = 1267,
    ErDropUser = 1268,
    ErRevokeGrants = 1269,
    ErCantAggregate3Collations = 1270,
    ErCantAggregateNcollations = 1271,
    ErVariableIsNotStruct = 1272,
    ErUnknownCollation = 1273,
    ErSlaveIgnoredSslParams = 1274,
    ErServerIsInSecureAuthMode = 1275,
    ErWarnFieldResolved = 1276,
    ErBadSlaveUntilCond = 1277,
    ErMissingSkipSlave = 1278,
    ErUntilCondIgnored = 1279,
    ErWrongNameForIndex = 1280,
    ErWrongNameForCatalog = 1281,
    ErWarnQcResize = 1282,
    ErBadFtColumn = 1283,
    ErUnknownKeyCache = 1284,
    ErWarnHostnameWontWork = 1285,
    ErUnknownStorageEngine = 1286,
    ErWarnDeprecatedSyntax = 1287,
    ErNonUpdatableTable = 1288,
    ErFeatureDisabled = 1289,
    ErOptionPreventsStatement = 1290,
    ErDuplicatedValueInType = 1291,
    ErTruncatedWrongValue = 1292,
    ErTooMuchAutoTimestampCols = 1293,
    ErInvalidOnUpdate = 1294,
    ErUnsupportedPs = 1295,
    ErGetErrmsg = 1296,
    ErGetTemporaryErrmsg = 1297,
    ErUnknownTimeZone = 1298,
    ErWarnInvalidTimestamp = 1299,
    ErInvalidCharacterString = 1300,
    ErWarnAllowedPacketOverflowed = 1301,
    ErConflictingDeclarations = 1302,
    ErSpNoRecursiveCreate = 1303,
    ErSpAlreadyExists = 1304,
    ErSpDoesNotExist = 1305,
    ErSpDropFailed = 1306,
    ErSpStoreFailed = 1307,
    ErSpLilabelMismatch = 1308,
    ErSpLabelRedefine = 1309,
    ErSpLabelMismatch = 1310,
    ErSpUninitVar = 1311,
    ErSpBadselect = 1312,
    ErSpBadreturn = 1313,
    ErSpBadstatement = 1314,
    ErUpdateLogDeprecatedIgnored = 1315,
    ErUpdateLogDeprecatedTranslated = 1316,
    ErQueryInterrupted = 1317,
    ErSpWrongNoOfArgs = 1318,
    ErSpCondMismatch = 1319,
    ErSpNoreturn = 1320,
    ErSpNoreturnend = 1321,
    ErSpBadCursorQuery = 1322,
    ErSpBadCursorSelect = 1323,
    ErSpCursorMismatch = 1324,
    ErSpCursorAlreadyOpen = 1325,
    ErSpCursorNotOpen = 1326,
    ErSpUndeclaredVar = 1327,
    ErSpWrongNoOfFetchArgs = 1328,
    ErSpFetchNoData = 1329,
    ErSpDupParam = 1330,
    ErSpDupVar = 1331,
    ErSpDupCond = 1332,
    ErSpDupCurs = 1333,
    ErSpCantAlter = 1334,
    ErSpSubselectNyi = 1335,
    ErStmtNotAllowedInSfOrTrg = 1336,
    ErSpVarcondAfterCurshndlr = 1337,
    ErSpCursorAfterHandler = 1338,
    ErSpCaseNotFound = 1339,
    ErFparserTooBigFile = 1340,
    ErFparserBadHeader = 1341,
    ErFparserEofInComment = 1342,
    ErFparserErrorInParameter = 1343,
    ErFparserEofInUnknownParameter = 1344,
    ErViewNoExplain = 1345,
    ErFrmUnknownType = 1346,
    ErWrongObject = 1347,
    ErNonupdateableColumn = 1348,
    ErViewSelectDerived = 1349,
    ErViewSelectClause = 1350,
    ErViewSelectVariable = 1351,
    ErViewSelectTmptable = 1352,
    ErViewWrongList = 1353,
    ErWarnViewMerge = 1354,
    ErWarnViewWithoutKey = 1355,
    ErViewInvalid = 1356,
    ErSpNoDropSp = 1357,
    ErSpGotoInHndlr = 1358,
    ErTrgAlreadyExists = 1359,
    ErTrgDoesNotExist = 1360,
    ErTrgOnViewOrTempTable = 1361,
    ErTrgCantChangeRow = 1362,
    ErTrgNoSuchRowInTrg = 1363,
    ErNoDefaultForField = 1364,
    ErDivisionByZer = 1365,
    ErTruncatedWrongValueForField = 1366,
    ErIllegalValueForType = 1367,
    ErViewNonupdCheck = 1368,
    ErViewCheckFailed = 1369,
    ErProcaccessDeniedError = 1370,
    ErRelayLogFail = 1371,
    ErPasswdLength = 1372,
    ErUnknownTargetBinlog = 1373,
    ErIoErrLogIndexRead = 1374,
    ErBinlogPurgeProhibited = 1375,
    ErFseekFail = 1376,
    ErBinlogPurgeFatalErr = 1377,
    ErLogInUse = 1378,
    ErLogPurgeUnknownErr = 1379,
    ErRelayLogInit = 1380,
    ErNoBinaryLogging = 1381,
    ErReservedSyntax = 1382,
    ErWsasFailed = 1383,
    ErDiffGroupsProc = 1384,
    ErNoGroupForProc = 1385,
    ErOrderWithProc = 1386,
    ErLoggingProhibitChangingOf = 1387,
    ErNoFileMapping = 1388,
    ErWrongMagic = 1389,
    ErPsManyParam = 1390,
    ErKeyPart0 = 1391,
    ErViewChecksum = 1392,
    ErViewMultiupdate = 1393,
    ErViewNoInsertFieldList = 1394,
    ErViewDeleteMergeView = 1395,
    ErCannotUser = 1396,
    ErXaerNota = 1397,
    ErXaerInval = 1398,
    ErXaerRmfail = 1399,
    ErXaerOutside = 1400,
    ErXaerRmerr = 1401,
    ErXaRbrollback = 1402,
    ErNonexistingProcGrant = 1403,
    ErProcAutoGrantFail = 1404,
    ErProcAutoRevokeFail = 1405,
    ErDataTooLong = 1406,
    ErSpBadSqlstate = 1407,
    ErStartup = 1408,
    ErLoadFromFixedSizeRowsToVar = 1409,
    ErCantCreateUserWithGrant = 1410,
    ErWrongValueForType = 1411,
    ErTableDefChanged = 1412,
    ErSpDupHandler = 1413,
    ErSpNotVarArg = 1414,
    ErSpNoRetset = 1415,
    ErCantCreateGeometryObject = 1416,
    ErFailedRoutineBreakBinlog = 1417,
    ErBinlogUnsafeRoutine = 1418,
    ErBinlogCreateRoutineNeedSuper = 1419,
    ErExecStmtWithOpenCursor = 1420,
    ErStmtHasNoOpenCursor = 1421,
    ErCommitNotAllowedInSfOrTrg = 1422,
    ErNoDefaultForViewField = 1423,
    ErSpNoRecursion = 1424,
    ErTooBigScale = 1425,
    ErTooBigPrecision = 1426,
    ErMBiggerThanD = 1427,
    ErWrongLockOfSystemTable = 1428,
    ErConnectToForeignDataSource = 1429,
    ErQueryOnForeignDataSource = 1430,
    ErForeignDataSourceDoesntExist = 1431,
    ErForeignDataStringInvalidCantCreate = 1432,
    ErForeignDataStringInvalid = 1433,
    ErCantCreateFederatedTable = 1434,
    ErTrgInWrongSchema = 1435,
    ErStackOverrunNeedMore = 1436,
    ErTooLongBody = 1437,
    ErWarnCantDropDefaultKeycache = 1438,
    ErTooBigDisplaywidth = 1439,
    ErXaerDupid = 1440,
    ErDatetimeFunctionOverflow = 1441,
    ErCantUpdateUsedTableInSfOrTrg = 1442,
    ErViewPreventUpdate = 1443,
    ErPsNoRecursion = 1444,
    ErSpCantSetAutocommit = 1445,
    ErMalformedDefiner = 1446,
    ErViewFrmNoUser = 1447,
    ErViewOtherUser = 1448,
    ErNoSuchUser = 1449,
    ErForbidSchemaChange = 1450,
    ErRowIsReferenced2 = 1451,
    ErNoReferencedRow2 = 1452,
    ErSpBadVarShadow = 1453,
    ErTrgNoDefiner = 1454,
    ErOldFileFormat = 1455,
    ErSpRecursionLimit = 1456,
    ErSpProcTableCorrupt = 1457,
    ErSpWrongName = 1458,
    ErTableNeedsUpgrade = 1459,
    ErSpNoAggregate = 1460,
    ErMaxPreparedStmtCountReached = 1461,
    ErViewRecursive = 1462,
    ErNonGroupingFieldUsed = 1463,
    ErTableCantHandleSpkeys = 1464,
    ErNoTriggersOnSystemSchema = 1465,
    ErRemovedSpaces = 1466,
    ErAutoincReadFailed = 1467,
    ErUsername = 1468,
    ErHostname = 1469,
    ErWrongStringLength = 1470,
    ErNonInsertableTable = 1471,
    ErAdminWrongMrgTable = 1472,
    ErTooHighLevelOfNestingForSelect = 1473,
    ErNameBecomesEmpty = 1474,
    ErAmbiguousFieldTerm = 1475,
    ErForeignServerExists = 1476,
    ErForeignServerDoesntExist = 1477,
    ErIllegalHaCreateOption = 1478,
    ErPartitionRequiresValuesError = 1479,
    ErPartitionWrongValuesError = 1480,
    ErPartitionMaxvalueError = 1481,
    ErPartitionSubpartitionError = 1482,
    ErPartitionSubpartMixError = 1483,
    ErPartitionWrongNoPartError = 1484,
    ErPartitionWrongNoSubpartError = 1485,
    ErConstExprInPartitionFuncError = 1486,
    // Duplicate error code
    // ErWrongExprInPartitionFuncError = 1486,
    ErNoConstExprInRangeOrListError = 1487,
    ErFieldNotFoundPartError = 1488,
    ErListOfFieldsOnlyInHashError = 1489,
    ErInconsistentPartitionInfoError = 1490,
    ErPartitionFuncNotAllowedError = 1491,
    ErPartitionsMustBeDefinedError = 1492,
    ErRangeNotIncreasingError = 1493,
    ErInconsistentTypeOfFunctionsError = 1494,
    ErMultipleDefConstInListPartError = 1495,
    ErPartitionEntryError = 1496,
    ErMixHandlerError = 1497,
    ErPartitionNotDefinedError = 1498,
    ErTooManyPartitionsError = 1499,
    ErSubpartitionError = 1500,
    ErCantCreateHandlerFile = 1501,
    ErBlobFieldInPartFuncError = 1502,
    ErUniqueKeyNeedAllFieldsInPf = 1503,
    ErNoPartsError = 1504,
    ErPartitionMgmtOnNonpartitioned = 1505,
    ErForeignKeyOnPartitioned = 1506,
    ErDropPartitionNonExistent = 1507,
    ErDropLastPartition = 1508,
    ErCoalesceOnlyOnHashPartition = 1509,
    ErReorgHashOnlyOnSameN = 1510,
    ErReorgNoParamError = 1511,
    ErOnlyOnRangeListPartition = 1512,
    ErAddPartitionSubpartError = 1513,
    ErAddPartitionNoNewPartition = 1514,
    ErCoalescePartitionNoPartition = 1515,
    ErReorgPartitionNotExist = 1516,
    ErSameNamePartition = 1517,
    ErNoBinlogError = 1518,
    ErConsecutiveReorgPartitions = 1519,
    ErReorgOutsideRange = 1520,
    ErPartitionFunctionFailure = 1521,
    ErPartStateError = 1522,
    ErLimitedPartRange = 1523,
    ErPluginIsNotLoaded = 1524,
    ErWrongValue = 1525,
    ErNoPartitionForGivenValue = 1526,
    ErFilegroupOptionOnlyOnce = 1527,
    ErCreateFilegroupFailed = 1528,
    ErDropFilegroupFailed = 1529,
    ErTablespaceAutoExtendError = 1530,
    ErWrongSizeNumber = 1531,
    ErSizeOverflowError = 1532,
    ErAlterFilegroupFailed = 1533,
    ErBinlogRowLoggingFailed = 1534,
    ErBinlogRowWrongTableDef = 1535,
    ErBinlogRowRbrToSbr = 1536,
    ErEventAlreadyExists = 1537,
    ErEventStoreFailed = 1538,
    ErEventDoesNotExist = 1539,
    ErEventCantAlter = 1540,
    ErEventDropFailed = 1541,
    ErEventIntervalNotPositiveOrTooBig = 1542,
    ErEventEndsBeforeStarts = 1543,
    ErEventExecTimeInThePast = 1544,
    ErEventOpenTableFailed = 1545,
    ErEventNeitherMExprNorMAt = 1546,
    ErColCountDoesntMatchCorrupted = 1547,
    ErCannotLoadFromTable = 1548,
    ErEventCannotDelete = 1549,
    ErEventCompileError = 1550,
    ErEventSameName = 1551,
    ErEventDataTooLong = 1552,
    ErDropIndexFk = 1553,
    ErWarnDeprecatedSyntaxWithVer = 1554,
    ErCantWriteLockLogTable = 1555,
    ErCantLockLogTable = 1556,
    ErForeignDuplicateKey = 1557,
    ErColCountDoesntMatchPleaseUpdate = 1558,
    ErTempTablePreventsSwitchOutOfRbr = 1559,
    ErStoredFunctionPreventsSwitchBinlogFormat = 1560,
    ErNdbCantSwitchBinlogFormat = 1561,
    ErPartitionNoTemporary = 1562,
    ErPartitionConstDomainError = 1563,
    ErPartitionFunctionIsNotAllowed = 1564,
    ErDdlLogError = 1565,
    ErNullInValuesLessThan = 1566,
    ErWrongPartitionName = 1567,
    ErCantChangeTxIsolation = 1568,
    ErDupEntryAutoincrementCase = 1569,
    ErEventModifyQueueError = 1570,
    ErEventSetVarError = 1571,
    ErPartitionMergeError = 1572,
    ErCantActivateLog = 1573,
    ErRbrNotAvailable = 1574,
    ErBase64DecodeError = 1575,
    ErEventRecursionForbidden = 1576,
    ErEventsDbError = 1577,
    ErOnlyIntegersAllowed = 1578,
    ErUnsuportedLogEngine = 1579,
    ErBadLogStatement = 1580,
    ErCantRenameLogTable = 1581,
    ErWrongParamcountToNativeFct = 1582,
    ErWrongParametersToNativeFct = 1583,
    ErWrongParametersToStoredFct = 1584,
    ErNativeFctNameCollision = 1585,
    ErDupEntryWithKeyName = 1586,
    ErBinlogPurgeEmfile = 1587,
    ErEventCannotCreateInThePast = 1588,
    ErEventCannotAlterInThePast = 1589,
    ErSlaveIncident = 1590,
    ErNoPartitionForGivenValueSilent = 1591,
    ErBinlogUnsafeStatement = 1592,
    ErSlaveFatalError = 1593,
    ErSlaveRelayLogReadFailure = 1594,
    ErSlaveRelayLogWriteFailure = 1595,
    ErSlaveCreateEventFailure = 1596,
    ErSlaveMasterComFailure = 1597,
    ErBinlogLoggingImpossible = 1598,
    ErViewNoCreationCtx = 1599,
    ErViewInvalidCreationCtx = 1600,
    ErSrInvalidCreationCtx = 1601,
    ErTrgCorruptedFile = 1602,
    ErTrgNoCreationCtx = 1603,
    ErTrgInvalidCreationCtx = 1604,
    ErEventInvalidCreationCtx = 1605,
    ErTrgCantOpenTable = 1606,
    ErCantCreateSroutine = 1607,
    ErUnused11 = 1608,
    ErNoFormatDescriptionEvent = 1609,
    ErSlaveCorruptEvent = 1610,
    ErLoadDataInvalidColumn = 1611,
    ErLogPurgeNoFile = 1612,
    ErXaRbtimeout = 1613,
    ErXaRbdeadlock = 1614,
    ErNeedReprepare = 1615,
    ErDelayedNotSupported = 1616,
    WarnNoMasterInf = 1617,
    WarnOptionIgnored = 1618,
    WarnPluginDeleteBuiltin = 1619,
    WarnPluginBusy = 1620,
    ErVariableIsReadonly = 1621,
    ErWarnEngineTransactionRollback = 1622,
    ErSlaveHeartbeatFailure = 1623,
    ErSlaveHeartbeatValueOutOfRange = 1624,
    ErNdbReplicationSchemaError = 1625,
    ErConflictFnParseError = 1626,
    ErExceptionsWriteError = 1627,
    ErTooLongTableComment = 1628,
    ErTooLongFieldComment = 1629,
    ErFuncInexistentNameCollision = 1630,
    ErDatabaseName = 1631,
    ErTableName = 1632,
    ErPartitionName = 1633,
    ErSubpartitionName = 1634,
    ErTemporaryName = 1635,
    ErRenamedName = 1636,
    ErTooManyConcurrentTrxs = 1637,
    WarnNonAsciiSeparatorNotImplemented = 1638,
    ErDebugSyncTimeout = 1639,
    ErDebugSyncHitLimit = 1640,
    ErDupSignalSet = 1641,
    ErSignalWarn = 1642,
    ErSignalNotFound = 1643,
    ErSignalException = 1644,
    ErResignalWithoutActiveHandler = 1645,
    ErSignalBadConditionType = 1646,
    WarnCondItemTruncated = 1647,
    ErCondItemTooLong = 1648,
    ErUnknownLocale = 1649,
    ErSlaveIgnoreServerIds = 1650,
    ErQueryCacheDisabled = 1651,
    ErSameNamePartitionField = 1652,
    ErPartitionColumnListError = 1653,
    ErWrongTypeColumnValueError = 1654,
    ErTooManyPartitionFuncFieldsError = 1655,
    ErMaxvalueInValuesIn = 1656,
    ErTooManyValuesError = 1657,
    ErRowSinglePartitionFieldError = 1658,
    ErFieldTypeNotAllowedAsPartitionField = 1659,
    ErPartitionFieldsTooLong = 1660,
    ErBinlogRowEngineAndStmtEngine = 1661,
    ErBinlogRowModeAndStmtEngine = 1662,
    ErBinlogUnsafeAndStmtEngine = 1663,
    ErBinlogRowInjectionAndStmtEngine = 1664,
    ErBinlogStmtModeAndRowEngine = 1665,
    ErBinlogRowInjectionAndStmtMode = 1666,
    ErBinlogMultipleEngines = 1667,
    ErBinlogUnsafeLimit = 1668,
    ErBinlogUnsafeInsertDelayed = 1669,
    ErBinlogUnsafeSystemTable = 1670,
    ErBinlogUnsafeAutoincColumns = 1671,
    ErBinlogUnsafeUdf = 1672,
    ErBinlogUnsafeSystemVariable = 1673,
    ErBinlogUnsafeSystemFunction = 1674,
    ErBinlogUnsafeNontransAfterTrans = 1675,
    ErMessageAndStatement = 1676,
    ErSlaveConversionFailed = 1677,
    ErSlaveCantCreateConversion = 1678,
    ErInsideTransactionPreventsSwitchBinlogFormat = 1679,
    ErPathLength = 1680,
    ErWarnDeprecatedSyntaxNoReplacement = 1681,
    ErWrongNativeTableStructure = 1682,
    ErWrongPerfschemaUsage = 1683,
    ErWarnISSkippedTable = 1684,
    ErInsideTransactionPreventsSwitchBinlogDirect = 1685,
    ErStoredFunctionPreventsSwitchBinlogDirect = 1686,
    ErSpatialMustHaveGeomCol = 1687,
    ErTooLongIndexComment = 1688,
    ErLockAborted = 1689,
    ErDataOutOfRange = 1690,
    ErWrongSpvarTypeInLimit = 1691,
    ErBinlogUnsafeMultipleEngines = 1692,
    ErBinlogUnsafeMixedStatement = 1693,
    ErInsideTransactionPreventsSwitchSqlLogBin = 1694,
    ErStoredFunctionPreventsSwitchSqlLogBin = 1695,
    ErFailedReadFromParFile = 1696,
    ErValuesIsNotIntTypeError = 1697,
    ErAccessDeniedNoPasswordError = 1698,
    ErSetPasswordAuthPlugin = 1699,
    ErGrantPluginUserExists = 1700,
    ErTruncateIllegalFk = 1701,
    ErPluginIsPermanent = 1702,
    ErSlaveHeartbeatValueOutOfRangeMin = 1703,
    ErSlaveHeartbeatValueOutOfRangeMax = 1704,
    ErStmtCacheFull = 1705,
    ErMultiUpdateKeyConflict = 1706,
    ErTableNeedsRebuild = 1707,
    WarnOptionBelowLimit = 1708,
    ErIndexColumnTooLong = 1709,
    ErErrorInTriggerBody = 1710,
    ErErrorInUnknownTriggerBody = 1711,
    ErIndexCorrupt = 1712,
    ErUndoRecordTooBig = 1713,
    ErBinlogUnsafeInsertIgnoreSelect = 1714,
    ErBinlogUnsafeInsertSelectUpdate = 1715,
    ErBinlogUnsafeReplaceSelect = 1716,
    ErBinlogUnsafeCreateIgnoreSelect = 1717,
    ErBinlogUnsafeCreateReplaceSelect = 1718,
    ErBinlogUnsafeUpdateIgnore = 1719,
    ErPluginNoUninstall = 1720,
    ErPluginNoInstall = 1721,
    ErBinlogUnsafeWriteAutoincSelect = 1722,
    ErBinlogUnsafeCreateSelectAutoinc = 1723,
    ErBinlogUnsafeInsertTwoKeys = 1724,
    ErTableInFkCheck = 1725,
    ErUnsupportedEngine = 1726,
    ErBinlogUnsafeAutoincNotFirst = 1727,
    ErCannotLoadFromTableV2 = 1728,
    ErMasterDelayValueOutOfRange = 1729,
    ErOnlyFdAndRbrEventsAllowedInBinlogStatement = 1730,
    ErPartitionExchangeDifferentOption = 1731,
    ErPartitionExchangePartTable = 1732,
    ErPartitionExchangeTempTable = 1733,
    ErPartitionInsteadOfSubpartition = 1734,
    ErUnknownPartition = 1735,
    ErTablesDifferentMetadata = 1736,
    ErRowDoesNotMatchPartition = 1737,
    ErBinlogCacheSizeGreaterThanMax = 1738,
    ErWarnIndexNotApplicable = 1739,
    ErPartitionExchangeForeignKey = 1740,
    ErNoSuchKeyValue = 1741,
    ErRplInfoDataTooLong = 1742,
    ErNetworkReadEventChecksumFailure = 1743,
    ErBinlogReadEventChecksumFailure = 1744,
    ErBinlogStmtCacheSizeGreaterThanMax = 1745,
    ErCantUpdateTableInCreateTableSelect = 1746,
    ErPartitionClauseOnNonpartitioned = 1747,
    ErRowDoesNotMatchGivenPartitionSet = 1748,
    ErNoSuchPartitionUnused = 1749,
    ErChangeRplInfoRepositoryFailure = 1750,
    ErWarningNotCompleteRollbackWithCreatedTempTable = 1751,
    ErWarningNotCompleteRollbackWithDroppedTempTable = 1752,
    ErMtsFeatureIsNotSupported = 1753,
    ErMtsUpdatedDbsGreaterMax = 1754,
    ErMtsCantParallel = 1755,
    ErMtsInconsistentData = 1756,
    ErFulltextNotSupportedWithPartitioning = 1757,
    ErDaInvalidConditionNumber = 1758,
    ErInsecurePlainText = 1759,
    ErInsecureChangeMaster = 1760,
    ErForeignDuplicateKeyWithChildInfo = 1761,
    ErForeignDuplicateKeyWithoutChildInfo = 1762,
    ErSqlthreadWithSecureSlave = 1763,
    ErTableHasNoFt = 1764,
    ErVariableNotSettableInSfOrTrigger = 1765,
    ErVariableNotSettableInTransaction = 1766,
    ErGtidNextIsNotInGtidNextList = 1767,
    ErCantChangeGtidNextInTransactionWhenGtidNextListIsNull = 1768,
    ErSetStatementCannotInvokeFunction = 1769,
    ErGtidNextCantBeAutomaticIfGtidNextListIsNonNull = 1770,
    ErSkippingLoggedTransaction = 1771,
    ErMalformedGtidSetSpecification = 1772,
    ErMalformedGtidSetEncoding = 1773,
    ErMalformedGtidSpecification = 1774,
    ErGnoExhausted = 1775,
    ErBadSlaveAutoPosition = 1776,
    ErAutoPositionRequiresGtidModeOn = 1777,
    ErCantDoImplicitCommitInTrxWhenGtidNextIsSet = 1778,
    ErGtidMode2Or3RequiresDisableGtidUnsafeStatementsOn = 1779,
    // https://mariadb.com/kb/en/library/mariadb-error-codes/ duplicate error code
    // ErGtidMode_2Or_3RequiresEnforceGtidConsistencyOn = 1779,
    ErGtidModeRequiresBinlog = 1780,
    ErCantSetGtidNextToGtidWhenGtidModeIsOff = 1781,
    ErCantSetGtidNextToAnonymousWhenGtidModeIsOn = 1782,
    ErCantSetGtidNextListToNonNullWhenGtidModeIsOff = 1783,
    ErFoundGtidEventWhenGtidModeIsOff = 1784,
    ErGtidUnsafeNonTransactionalTable = 1785,
    ErGtidUnsafeCreateSelect = 1786,
    ErGtidUnsafeCreateDropTemporaryTableInTransaction = 1787,
    ErGtidModeCanOnlyChangeOneStepAtATime = 1788,
    ErMasterHasPurgedRequiredGtids = 1789,
    ErCantSetGtidNextWhenOwningGtid = 1790,
    ErUnknownExplainFormat = 1791,
    ErCantExecuteInReadOnlyTransaction = 1792,
    ErTooLongTablePartitionComment = 1793,
    ErSlaveConfiguration = 1794,
    ErInnodbFtLimit = 1795,
    ErInnodbNoFtTempTable = 1796,
    ErInnodbFtWrongDocidColumn = 1797,
    ErInnodbFtWrongDocidIndex = 1798,
    ErInnodbOnlineLogTooBig = 1799,
    ErUnknownAlterAlgorithm = 1800,
    ErUnknownAlterLock = 1801,
    ErMtsChangeMasterCantRunWithGaps = 1802,
    ErMtsRecoveryFailure = 1803,
    ErMtsResetWorkers = 1804,
    ErColCountDoesntMatchCorruptedV2 = 1805,
    ErSlaveSilentRetryTransaction = 1806,
    ErDiscardFkChecksRunning = 1807,
    ErTableSchemaMismatch = 1808,
    ErTableInSystemTablespace = 1809,
    ErIoReadError = 1810,
    ErIoWriteError = 1811,
    ErTablespaceMissing = 1812,
    ErTablespaceExists = 1813,
    ErTablespaceDiscarded = 1814,
    ErInternalError = 1815,
    ErInnodbImportError = 1816,
    ErInnodbIndexCorrupt = 1817,
    ErInvalidYearColumnLength = 1818,
    ErNotValidPassword = 1819,
    ErMustChangePassword = 1820,
    ErFkNoIndexChild = 1821,
    ErFkNoIndexParent = 1822,
    ErFkFailAddSystem = 1823,
    ErFkCannotOpenParent = 1824,
    ErFkIncorrectOption = 1825,
    ErFkDupName = 1826,
    ErPasswordFormat = 1827,
    ErFkColumnCannotDrop = 1828,
    ErFkColumnCannotDropChild = 1829,
    ErFkColumnNotNull = 1830,
    ErDupIndex = 1831,
    ErFkColumnCannotChange = 1832,
    ErFkColumnCannotChangeChild = 1833,
    ErFkCannotDeleteParent = 1834,
    ErMalformedPacket = 1835,
    ErReadOnlyMode = 1836,
    ErGtidNextTypeUndefinedGroup = 1837,
    ErVariableNotSettableInSp = 1838,
    ErCantSetGtidPurgedWhenGtidModeIsOff = 1839,
    ErCantSetGtidPurgedWhenGtidExecutedIsNotEmpty = 1840,
    ErCantSetGtidPurgedWhenOwnedGtidsIsNotEmpty = 1841,
    ErGtidPurgedWasChanged = 1842,
    ErGtidExecutedWasChanged = 1843,
    ErBinlogStmtModeAndNoReplTables = 1844,
    ErAlterOperationNotSupported = 1845,
    ErAlterOperationNotSupportedReason = 1846,
    ErAlterOperationNotSupportedReasonCopy = 1847,
    ErAlterOperationNotSupportedReasonPartition = 1848,
    ErAlterOperationNotSupportedReasonFkRename = 1849,
    ErAlterOperationNotSupportedReasonColumnType = 1850,
    ErAlterOperationNotSupportedReasonFkCheck = 1851,
    ErAlterOperationNotSupportedReasonIgnore = 1852,
    ErAlterOperationNotSupportedReasonNopk = 1853,
    ErAlterOperationNotSupportedReasonAutoinc = 1854,
    ErAlterOperationNotSupportedReasonHiddenFts = 1855,
    ErAlterOperationNotSupportedReasonChangeFts = 1856,
    ErAlterOperationNotSupportedReasonFts = 1857,
    ErSqlSlaveSkipCounterNotSettableInGtidMode = 1858,
    ErDupUnknownInIndex = 1859,
    ErIdentCausesTooLongPath = 1860,
    ErAlterOperationNotSupportedReasonNotNull = 1861,
    ErMustChangePasswordLogin = 1862,
    ErRowInWrongPartition = 1863,
    ErMtsEventBiggerPendingJobsSizeMax = 1864,
    ErInnodbNoFtUsesParser = 1865,
    ErBinlogLogicalCorruption = 1866,
    ErWarnPurgeLogInUse = 1867,
    ErWarnPurgeLogIsActive = 1868,
    ErAutoIncrementConflict = 1869,
    WarnOnBlockholeInRbr = 1870,
    ErSlaveMiInitRepository = 1871,
    ErSlaveRliInitRepository = 1872,
    ErAccessDeniedChangeUserError = 1873,
    ErInnodbReadOnly = 1874,
    ErStopSlaveSqlThreadTimeout = 1875,
    ErStopSlaveIoThreadTimeout = 1876,
    ErTableCorrupt = 1877,
    ErTempFileWriteFailure = 1878,
    ErInnodbFtAuxNotHexId = 1879,
    ErOldTemporalsUpgraded = 1880,
    ErInnodbForcedRecovery = 1881,
    ErAesInvalidIv = 1882,
    ErPluginCannotBeUninstalled = 1883,
    ErGtidUnsafeBinlogSplittableStatementAndGtidGroup = 1884,
    ErSlaveHasMoreGtidsThanMaster = 1885,
    ErVcolBasedOnVcol = 1900,
    ErVirtualColumnFunctionIsNotAllowed = 1901,
    ErDataConversionErrorForVirtualColumn = 1902,
    ErPrimaryKeyBasedOnVirtualColumn = 1903,
    ErKeyBasedOnGeneratedVirtualColumn = 1904,
    ErWrongFkOptionForVirtualColumn = 1905,
    ErWarningNonDefaultValueForVirtualColumn = 1906,
    ErUnsupportedActionOnVirtualColumn = 1907,
    ErConstExprInVcol = 1908,
    ErRowExprForVcol = 1909,
    ErUnsupportedEngineForVirtualColumns = 1910,
    ErUnknownOption = 1911,
    ErBadOptionValue = 1912,
    // Duplicate error code
    // ErNetworkReadEventChecksumFailure = 1913,
    // ErBinlogReadEventChecksumFailure = 1914,
    ErCantDoOnline = 1915,
    ErDataOverflow = 1916,
    ErDataTruncated = 1917,
    ErBadData = 1918,
    ErDynColWrongFormat = 1919,
    ErDynColImplementationLimit = 1920,
    ErDynColData = 1921,
    ErDynColWrongCharset = 1922,
    ErIllegalSubqueryOptimizerSwitches = 1923,
    ErQueryCacheIsDisabled = 1924,
    ErQueryCacheIsGlobalyDisabled = 1925,
    ErViewOrderbyIgnored = 1926,
    ErConnectionKilled = 1927,
    // ErInternalError = 1928,
    ErInsideTransactionPreventsSwitchSkipReplication = 1929,
    ErStoredFunctionPreventsSwitchSkipReplication = 1930,
    ErQueryExceededRowsExaminedLimit = 1931,
    ErNoSuchTableInEngine = 1932,
    ErTargetNotExplainable = 1933,
    ErConnectionAlreadyExists = 1934,
    ErMasterLogPrefix = 1935,
    ErCantStartStopSlave = 1936,
    ErSlaveStarted = 1937,
    ErSlaveStopped = 1938,
    ErSqlDiscoverError = 1939,
    ErFailedGtidStateInit = 1940,
    ErIncorrectGtidState = 1941,
    ErCannotUpdateGtidState = 1942,
    ErDuplicateGtidDomain = 1943,
    ErGtidOpenTableFailed = 1944,
    ErGtidPositionNotFoundInBinlog = 1945,
    ErCannotLoadSlaveGtidState = 1946,
    ErMasterGtidPosConflictsWithBinlog = 1947,
    ErMasterGtidPosMissingDomain = 1948,
    ErUntilRequiresUsingGtid = 1949,
    ErGtidStrictOutOfOrder = 1950,
    ErGtidStartFromBinlogHole = 1951,
    ErSlaveUnexpectedMasterSwitch = 1952,
    ErInsideTransactionPreventsSwitchGtidDomainIdSeqNo = 1953,
    ErStoredFunctionPreventsSwitchGtidDomainIdSeqNo = 1954,
    ErGtidPositionNotFoundInBinlog2 = 1955,
    ErBinlogMustBeEmpty = 1956,
    ErNoSuchQuery = 1957,
    ErBadBase64Data = 1958,
    ErInvalidRole = 1959,
    ErInvalidCurrentUser = 1960,
    ErCannotGrantRole = 1961,
    ErCannotRevokeRole = 1962,
    ErChangeSlaveParallelThreadsActive = 1963,
    ErPriorCommitFailed = 1964,
    ErItIsAView = 1965,
    ErSlaveSkipNotInGtid = 1966,
    ErTableDefinitionTooBig = 1967,
    ErPluginInstalled = 1968,
    ErStatementTimeout = 1969,
    ErSubqueriesNotSupported = 1970,
    ErSetStatementNotSupported = 1971,
    ErUnused17 = 1972,
    ErUserCreateExists = 1973,
    ErUserDropExists = 1974,
    ErRoleCreateExists = 1975,
    ErRoleDropExists = 1976,
    ErCannotConvertCharacter = 1977,
    ErInvalidDefaultValueForField = 1978,
    ErKillQueryDeniedError = 1979,
    ErNoEisForField = 1980,
    ErWarnAggfuncDependence = 1981,
}

impl Default for ErrorCode {
    fn default() -> Self {
        ErrorCode::ErDefault
    }
}
