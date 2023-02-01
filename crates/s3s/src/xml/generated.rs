use super::*;

use crate::dto::*;

use std::io::Write;

impl SerializeContent for AbortIncompleteMultipartUpload {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("DaysAfterInitiation", &self.days_after_initiation)?;
        Ok(())
    }
}

impl SerializeContent for AccessControlTranslation {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("Owner", &self.owner)?;
        Ok(())
    }
}

impl SerializeContent for AnalyticsAndOperator {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        if let Some(iter) = &self.tags {
            s.flattened_list("Tag", iter)?;
        }
        Ok(())
    }
}

impl SerializeContent for AnalyticsConfiguration {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.filter {
            s.content("Filter", val)?;
        }
        s.content("Id", &self.id)?;
        s.content("StorageClassAnalysis", &self.storage_class_analysis)?;
        Ok(())
    }
}

impl SerializeContent for AnalyticsExportDestination {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("S3BucketDestination", &self.s3_bucket_destination)?;
        Ok(())
    }
}

impl SerializeContent for AnalyticsFilter {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        match self {
            Self::And(x) => s.content("And", x),
            Self::Prefix(x) => s.content("Prefix", x),
            Self::Tag(x) => s.content("Tag", x),
        }
    }
}

impl SerializeContent for AnalyticsS3BucketDestination {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("Bucket", &self.bucket)?;
        if let Some(ref val) = self.bucket_account_id {
            s.content("BucketAccountId", val)?;
        }
        s.content("Format", &self.format)?;
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for AnalyticsS3ExportFileFormat {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for Bucket {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.creation_date {
            s.timestamp("CreationDate", val, TimestampFormat::DateTime)?;
        }
        if let Some(ref val) = self.name {
            s.content("Name", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for BucketAccelerateStatus {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for BucketLocationConstraint {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for BucketLogsPermission {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for BucketVersioningStatus {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for CORSRule {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(iter) = &self.allowed_headers {
            s.flattened_list("AllowedHeader", iter)?;
        }
        {
            let iter = &self.allowed_methods;
            s.flattened_list("AllowedMethod", iter)?;
        }
        {
            let iter = &self.allowed_origins;
            s.flattened_list("AllowedOrigin", iter)?;
        }
        if let Some(iter) = &self.expose_headers {
            s.flattened_list("ExposeHeader", iter)?;
        }
        if let Some(ref val) = self.id {
            s.content("ID", val)?;
        }
        s.content("MaxAgeSeconds", &self.max_age_seconds)?;
        Ok(())
    }
}

impl SerializeContent for Checksum {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.checksum_crc32 {
            s.content("ChecksumCRC32", val)?;
        }
        if let Some(ref val) = self.checksum_crc32c {
            s.content("ChecksumCRC32C", val)?;
        }
        if let Some(ref val) = self.checksum_sha1 {
            s.content("ChecksumSHA1", val)?;
        }
        if let Some(ref val) = self.checksum_sha256 {
            s.content("ChecksumSHA256", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for ChecksumAlgorithm {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for CommonPrefix {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for CompleteMultipartUploadOutput {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.bucket {
            s.content("Bucket", val)?;
        }
        if let Some(ref val) = self.checksum_crc32 {
            s.content("ChecksumCRC32", val)?;
        }
        if let Some(ref val) = self.checksum_crc32c {
            s.content("ChecksumCRC32C", val)?;
        }
        if let Some(ref val) = self.checksum_sha1 {
            s.content("ChecksumSHA1", val)?;
        }
        if let Some(ref val) = self.checksum_sha256 {
            s.content("ChecksumSHA256", val)?;
        }
        if let Some(ref val) = self.e_tag {
            s.content("ETag", val)?;
        }
        if let Some(ref val) = self.key {
            s.content("Key", val)?;
        }
        if let Some(ref val) = self.location {
            s.content("Location", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for Condition {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.http_error_code_returned_equals {
            s.content("HttpErrorCodeReturnedEquals", val)?;
        }
        if let Some(ref val) = self.key_prefix_equals {
            s.content("KeyPrefixEquals", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for CopyObjectResult {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.checksum_crc32 {
            s.content("ChecksumCRC32", val)?;
        }
        if let Some(ref val) = self.checksum_crc32c {
            s.content("ChecksumCRC32C", val)?;
        }
        if let Some(ref val) = self.checksum_sha1 {
            s.content("ChecksumSHA1", val)?;
        }
        if let Some(ref val) = self.checksum_sha256 {
            s.content("ChecksumSHA256", val)?;
        }
        if let Some(ref val) = self.e_tag {
            s.content("ETag", val)?;
        }
        if let Some(ref val) = self.last_modified {
            s.timestamp("LastModified", val, TimestampFormat::DateTime)?;
        }
        Ok(())
    }
}

impl SerializeContent for CopyPartResult {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.checksum_crc32 {
            s.content("ChecksumCRC32", val)?;
        }
        if let Some(ref val) = self.checksum_crc32c {
            s.content("ChecksumCRC32C", val)?;
        }
        if let Some(ref val) = self.checksum_sha1 {
            s.content("ChecksumSHA1", val)?;
        }
        if let Some(ref val) = self.checksum_sha256 {
            s.content("ChecksumSHA256", val)?;
        }
        if let Some(ref val) = self.e_tag {
            s.content("ETag", val)?;
        }
        if let Some(ref val) = self.last_modified {
            s.timestamp("LastModified", val, TimestampFormat::DateTime)?;
        }
        Ok(())
    }
}

impl SerializeContent for CreateMultipartUploadOutput {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.bucket {
            s.content("Bucket", val)?;
        }
        if let Some(ref val) = self.key {
            s.content("Key", val)?;
        }
        if let Some(ref val) = self.upload_id {
            s.content("UploadId", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for DefaultRetention {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("Days", &self.days)?;
        if let Some(ref val) = self.mode {
            s.content("Mode", val)?;
        }
        s.content("Years", &self.years)?;
        Ok(())
    }
}

impl SerializeContent for DeleteMarkerEntry {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("IsLatest", &self.is_latest)?;
        if let Some(ref val) = self.key {
            s.content("Key", val)?;
        }
        if let Some(ref val) = self.last_modified {
            s.timestamp("LastModified", val, TimestampFormat::DateTime)?;
        }
        if let Some(ref val) = self.owner {
            s.content("Owner", val)?;
        }
        if let Some(ref val) = self.version_id {
            s.content("VersionId", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for DeleteMarkerReplication {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.status {
            s.content("Status", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for DeleteMarkerReplicationStatus {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for DeleteObjectsOutput {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(iter) = &self.deleted {
            s.flattened_list("Deleted", iter)?;
        }
        if let Some(iter) = &self.errors {
            s.flattened_list("Error", iter)?;
        }
        Ok(())
    }
}

impl SerializeContent for DeletedObject {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("DeleteMarker", &self.delete_marker)?;
        if let Some(ref val) = self.delete_marker_version_id {
            s.content("DeleteMarkerVersionId", val)?;
        }
        if let Some(ref val) = self.key {
            s.content("Key", val)?;
        }
        if let Some(ref val) = self.version_id {
            s.content("VersionId", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for Destination {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.access_control_translation {
            s.content("AccessControlTranslation", val)?;
        }
        if let Some(ref val) = self.account {
            s.content("Account", val)?;
        }
        s.content("Bucket", &self.bucket)?;
        if let Some(ref val) = self.encryption_configuration {
            s.content("EncryptionConfiguration", val)?;
        }
        if let Some(ref val) = self.metrics {
            s.content("Metrics", val)?;
        }
        if let Some(ref val) = self.replication_time {
            s.content("ReplicationTime", val)?;
        }
        if let Some(ref val) = self.storage_class {
            s.content("StorageClass", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for EncodingType {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for EncryptionConfiguration {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.replica_kms_key_id {
            s.content("ReplicaKmsKeyID", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for Error {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.code {
            s.content("Code", val)?;
        }
        if let Some(ref val) = self.key {
            s.content("Key", val)?;
        }
        if let Some(ref val) = self.message {
            s.content("Message", val)?;
        }
        if let Some(ref val) = self.version_id {
            s.content("VersionId", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for ErrorDocument {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("Key", &self.key)?;
        Ok(())
    }
}

impl SerializeContent for EventBridgeConfiguration {
    fn serialize_content<W: Write>(&self, _: &mut Serializer<W>) -> SerResult {
        Ok(())
    }
}

impl SerializeContent for ExistingObjectReplication {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("Status", &self.status)?;
        Ok(())
    }
}

impl SerializeContent for ExistingObjectReplicationStatus {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for ExpirationStatus {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for FilterRule {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.name {
            s.content("Name", val)?;
        }
        if let Some(ref val) = self.value {
            s.content("Value", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for FilterRuleName {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for GetBucketAccelerateConfigurationOutput {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.status {
            s.content("Status", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for GetBucketAclOutput {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(iter) = &self.grants {
            s.list("AccessControlList", "Grant", iter)?;
        }
        if let Some(ref val) = self.owner {
            s.content("Owner", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for GetBucketCorsOutput {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(iter) = &self.cors_rules {
            s.flattened_list("CORSRule", iter)?;
        }
        Ok(())
    }
}

impl SerializeContent for GetBucketLifecycleConfigurationOutput {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(iter) = &self.rules {
            s.flattened_list("Rule", iter)?;
        }
        Ok(())
    }
}

impl SerializeContent for GetBucketLocationOutput {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.location_constraint {
            s.content("LocationConstraint", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for GetBucketLoggingOutput {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.logging_enabled {
            s.content("LoggingEnabled", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for GetBucketNotificationConfigurationOutput {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.event_bridge_configuration {
            s.content("EventBridgeConfiguration", val)?;
        }
        if let Some(iter) = &self.lambda_function_configurations {
            s.flattened_list("CloudFunctionConfiguration", iter)?;
        }
        if let Some(iter) = &self.queue_configurations {
            s.flattened_list("QueueConfiguration", iter)?;
        }
        if let Some(iter) = &self.topic_configurations {
            s.flattened_list("TopicConfiguration", iter)?;
        }
        Ok(())
    }
}

impl SerializeContent for GetBucketRequestPaymentOutput {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.payer {
            s.content("Payer", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for GetBucketTaggingOutput {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        {
            let iter = &self.tag_set;
            s.list("TagSet", "Tag", iter)?;
        }
        Ok(())
    }
}

impl SerializeContent for GetBucketVersioningOutput {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.mfa_delete {
            s.content("MfaDelete", val)?;
        }
        if let Some(ref val) = self.status {
            s.content("Status", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for GetBucketWebsiteOutput {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.error_document {
            s.content("ErrorDocument", val)?;
        }
        if let Some(ref val) = self.index_document {
            s.content("IndexDocument", val)?;
        }
        if let Some(ref val) = self.redirect_all_requests_to {
            s.content("RedirectAllRequestsTo", val)?;
        }
        if let Some(iter) = &self.routing_rules {
            s.list("RoutingRules", "RoutingRule", iter)?;
        }
        Ok(())
    }
}

impl SerializeContent for GetObjectAclOutput {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(iter) = &self.grants {
            s.list("AccessControlList", "Grant", iter)?;
        }
        if let Some(ref val) = self.owner {
            s.content("Owner", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for GetObjectAttributesOutput {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.checksum {
            s.content("Checksum", val)?;
        }
        if let Some(ref val) = self.e_tag {
            s.content("ETag", val)?;
        }
        if let Some(ref val) = self.object_parts {
            s.content("ObjectParts", val)?;
        }
        s.content("ObjectSize", &self.object_size)?;
        if let Some(ref val) = self.storage_class {
            s.content("StorageClass", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for GetObjectAttributesParts {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("IsTruncated", &self.is_truncated)?;
        s.content("MaxParts", &self.max_parts)?;
        if let Some(ref val) = self.next_part_number_marker {
            s.content("NextPartNumberMarker", val)?;
        }
        if let Some(ref val) = self.part_number_marker {
            s.content("PartNumberMarker", val)?;
        }
        if let Some(iter) = &self.parts {
            s.flattened_list("Part", iter)?;
        }
        s.content("PartsCount", &self.total_parts_count)?;
        Ok(())
    }
}

impl SerializeContent for GetObjectTaggingOutput {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        {
            let iter = &self.tag_set;
            s.list("TagSet", "Tag", iter)?;
        }
        Ok(())
    }
}

impl SerializeContent for Grant {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.grantee {
            s.content("Grantee", val)?;
        }
        if let Some(ref val) = self.permission {
            s.content("Permission", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for Grantee {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.display_name {
            s.content("DisplayName", val)?;
        }
        if let Some(ref val) = self.email_address {
            s.content("EmailAddress", val)?;
        }
        if let Some(ref val) = self.id {
            s.content("ID", val)?;
        }
        s.content("xsi:type", &self.type_)?;
        if let Some(ref val) = self.uri {
            s.content("URI", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for IndexDocument {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("Suffix", &self.suffix)?;
        Ok(())
    }
}

impl SerializeContent for Initiator {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.display_name {
            s.content("DisplayName", val)?;
        }
        if let Some(ref val) = self.id {
            s.content("ID", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for IntelligentTieringAccessTier {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for IntelligentTieringAndOperator {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        if let Some(iter) = &self.tags {
            s.flattened_list("Tag", iter)?;
        }
        Ok(())
    }
}

impl SerializeContent for IntelligentTieringConfiguration {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.filter {
            s.content("Filter", val)?;
        }
        s.content("Id", &self.id)?;
        s.content("Status", &self.status)?;
        {
            let iter = &self.tierings;
            s.flattened_list("Tiering", iter)?;
        }
        Ok(())
    }
}

impl SerializeContent for IntelligentTieringFilter {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.and {
            s.content("And", val)?;
        }
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        if let Some(ref val) = self.tag {
            s.content("Tag", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for IntelligentTieringStatus {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for InventoryConfiguration {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("Destination", &self.destination)?;
        if let Some(ref val) = self.filter {
            s.content("Filter", val)?;
        }
        s.content("Id", &self.id)?;
        s.content("IncludedObjectVersions", &self.included_object_versions)?;
        s.content("IsEnabled", &self.is_enabled)?;
        if let Some(iter) = &self.optional_fields {
            s.list("OptionalFields", "Field", iter)?;
        }
        s.content("Schedule", &self.schedule)?;
        Ok(())
    }
}

impl SerializeContent for InventoryDestination {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("S3BucketDestination", &self.s3_bucket_destination)?;
        Ok(())
    }
}

impl SerializeContent for InventoryEncryption {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.ssekms {
            s.content("SSE-KMS", val)?;
        }
        if let Some(ref val) = self.sses3 {
            s.content("SSE-S3", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for InventoryFilter {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("Prefix", &self.prefix)?;
        Ok(())
    }
}

impl SerializeContent for InventoryFormat {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for InventoryFrequency {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for InventoryIncludedObjectVersions {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for InventoryOptionalField {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for InventoryS3BucketDestination {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.account_id {
            s.content("AccountId", val)?;
        }
        s.content("Bucket", &self.bucket)?;
        if let Some(ref val) = self.encryption {
            s.content("Encryption", val)?;
        }
        s.content("Format", &self.format)?;
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for InventorySchedule {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("Frequency", &self.frequency)?;
        Ok(())
    }
}

impl SerializeContent for LambdaFunctionConfiguration {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        {
            let iter = &self.events;
            s.flattened_list("Event", iter)?;
        }
        if let Some(ref val) = self.filter {
            s.content("Filter", val)?;
        }
        if let Some(ref val) = self.id {
            s.content("Id", val)?;
        }
        s.content("CloudFunction", &self.lambda_function_arn)?;
        Ok(())
    }
}

impl SerializeContent for LifecycleExpiration {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.date {
            s.timestamp("Date", val, TimestampFormat::DateTime)?;
        }
        s.content("Days", &self.days)?;
        s.content("ExpiredObjectDeleteMarker", &self.expired_object_delete_marker)?;
        Ok(())
    }
}

impl SerializeContent for LifecycleRule {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.abort_incomplete_multipart_upload {
            s.content("AbortIncompleteMultipartUpload", val)?;
        }
        if let Some(ref val) = self.expiration {
            s.content("Expiration", val)?;
        }
        if let Some(ref val) = self.filter {
            s.content("Filter", val)?;
        }
        if let Some(ref val) = self.id {
            s.content("ID", val)?;
        }
        if let Some(ref val) = self.noncurrent_version_expiration {
            s.content("NoncurrentVersionExpiration", val)?;
        }
        if let Some(iter) = &self.noncurrent_version_transitions {
            s.flattened_list("NoncurrentVersionTransition", iter)?;
        }
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        s.content("Status", &self.status)?;
        if let Some(iter) = &self.transitions {
            s.flattened_list("Transition", iter)?;
        }
        Ok(())
    }
}

impl SerializeContent for LifecycleRuleAndOperator {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("ObjectSizeGreaterThan", &self.object_size_greater_than)?;
        s.content("ObjectSizeLessThan", &self.object_size_less_than)?;
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        if let Some(iter) = &self.tags {
            s.flattened_list("Tag", iter)?;
        }
        Ok(())
    }
}

impl SerializeContent for LifecycleRuleFilter {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        match self {
            Self::And(x) => s.content("And", x),
            Self::ObjectSizeGreaterThan(x) => s.content("ObjectSizeGreaterThan", x),
            Self::ObjectSizeLessThan(x) => s.content("ObjectSizeLessThan", x),
            Self::Prefix(x) => s.content("Prefix", x),
            Self::Tag(x) => s.content("Tag", x),
        }
    }
}

impl SerializeContent for ListBucketAnalyticsConfigurationsOutput {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(iter) = &self.analytics_configuration_list {
            s.flattened_list("AnalyticsConfiguration", iter)?;
        }
        if let Some(ref val) = self.continuation_token {
            s.content("ContinuationToken", val)?;
        }
        s.content("IsTruncated", &self.is_truncated)?;
        if let Some(ref val) = self.next_continuation_token {
            s.content("NextContinuationToken", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for ListBucketIntelligentTieringConfigurationsOutput {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.continuation_token {
            s.content("ContinuationToken", val)?;
        }
        if let Some(iter) = &self.intelligent_tiering_configuration_list {
            s.flattened_list("IntelligentTieringConfiguration", iter)?;
        }
        s.content("IsTruncated", &self.is_truncated)?;
        if let Some(ref val) = self.next_continuation_token {
            s.content("NextContinuationToken", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for ListBucketInventoryConfigurationsOutput {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.continuation_token {
            s.content("ContinuationToken", val)?;
        }
        if let Some(iter) = &self.inventory_configuration_list {
            s.flattened_list("InventoryConfiguration", iter)?;
        }
        s.content("IsTruncated", &self.is_truncated)?;
        if let Some(ref val) = self.next_continuation_token {
            s.content("NextContinuationToken", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for ListBucketMetricsConfigurationsOutput {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.continuation_token {
            s.content("ContinuationToken", val)?;
        }
        s.content("IsTruncated", &self.is_truncated)?;
        if let Some(iter) = &self.metrics_configuration_list {
            s.flattened_list("MetricsConfiguration", iter)?;
        }
        if let Some(ref val) = self.next_continuation_token {
            s.content("NextContinuationToken", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for ListBucketsOutput {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(iter) = &self.buckets {
            s.list("Buckets", "Bucket", iter)?;
        }
        if let Some(ref val) = self.owner {
            s.content("Owner", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for ListMultipartUploadsOutput {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.bucket {
            s.content("Bucket", val)?;
        }
        if let Some(iter) = &self.common_prefixes {
            s.flattened_list("CommonPrefixes", iter)?;
        }
        if let Some(ref val) = self.delimiter {
            s.content("Delimiter", val)?;
        }
        if let Some(ref val) = self.encoding_type {
            s.content("EncodingType", val)?;
        }
        s.content("IsTruncated", &self.is_truncated)?;
        if let Some(ref val) = self.key_marker {
            s.content("KeyMarker", val)?;
        }
        s.content("MaxUploads", &self.max_uploads)?;
        if let Some(ref val) = self.next_key_marker {
            s.content("NextKeyMarker", val)?;
        }
        if let Some(ref val) = self.next_upload_id_marker {
            s.content("NextUploadIdMarker", val)?;
        }
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        if let Some(ref val) = self.upload_id_marker {
            s.content("UploadIdMarker", val)?;
        }
        if let Some(iter) = &self.uploads {
            s.flattened_list("Upload", iter)?;
        }
        Ok(())
    }
}

impl SerializeContent for ListObjectVersionsOutput {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(iter) = &self.common_prefixes {
            s.flattened_list("CommonPrefixes", iter)?;
        }
        if let Some(iter) = &self.delete_markers {
            s.flattened_list("DeleteMarker", iter)?;
        }
        if let Some(ref val) = self.delimiter {
            s.content("Delimiter", val)?;
        }
        if let Some(ref val) = self.encoding_type {
            s.content("EncodingType", val)?;
        }
        s.content("IsTruncated", &self.is_truncated)?;
        if let Some(ref val) = self.key_marker {
            s.content("KeyMarker", val)?;
        }
        s.content("MaxKeys", &self.max_keys)?;
        if let Some(ref val) = self.name {
            s.content("Name", val)?;
        }
        if let Some(ref val) = self.next_key_marker {
            s.content("NextKeyMarker", val)?;
        }
        if let Some(ref val) = self.next_version_id_marker {
            s.content("NextVersionIdMarker", val)?;
        }
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        if let Some(ref val) = self.version_id_marker {
            s.content("VersionIdMarker", val)?;
        }
        if let Some(iter) = &self.versions {
            s.flattened_list("Version", iter)?;
        }
        Ok(())
    }
}

impl SerializeContent for ListObjectsOutput {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(iter) = &self.common_prefixes {
            s.flattened_list("CommonPrefixes", iter)?;
        }
        if let Some(iter) = &self.contents {
            s.flattened_list("Contents", iter)?;
        }
        if let Some(ref val) = self.delimiter {
            s.content("Delimiter", val)?;
        }
        if let Some(ref val) = self.encoding_type {
            s.content("EncodingType", val)?;
        }
        s.content("IsTruncated", &self.is_truncated)?;
        if let Some(ref val) = self.marker {
            s.content("Marker", val)?;
        }
        s.content("MaxKeys", &self.max_keys)?;
        if let Some(ref val) = self.name {
            s.content("Name", val)?;
        }
        if let Some(ref val) = self.next_marker {
            s.content("NextMarker", val)?;
        }
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for ListObjectsV2Output {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(iter) = &self.common_prefixes {
            s.flattened_list("CommonPrefixes", iter)?;
        }
        if let Some(iter) = &self.contents {
            s.flattened_list("Contents", iter)?;
        }
        if let Some(ref val) = self.continuation_token {
            s.content("ContinuationToken", val)?;
        }
        if let Some(ref val) = self.delimiter {
            s.content("Delimiter", val)?;
        }
        if let Some(ref val) = self.encoding_type {
            s.content("EncodingType", val)?;
        }
        s.content("IsTruncated", &self.is_truncated)?;
        s.content("KeyCount", &self.key_count)?;
        s.content("MaxKeys", &self.max_keys)?;
        if let Some(ref val) = self.name {
            s.content("Name", val)?;
        }
        if let Some(ref val) = self.next_continuation_token {
            s.content("NextContinuationToken", val)?;
        }
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        if let Some(ref val) = self.start_after {
            s.content("StartAfter", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for ListPartsOutput {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.bucket {
            s.content("Bucket", val)?;
        }
        if let Some(ref val) = self.checksum_algorithm {
            s.content("ChecksumAlgorithm", val)?;
        }
        if let Some(ref val) = self.initiator {
            s.content("Initiator", val)?;
        }
        s.content("IsTruncated", &self.is_truncated)?;
        if let Some(ref val) = self.key {
            s.content("Key", val)?;
        }
        s.content("MaxParts", &self.max_parts)?;
        if let Some(ref val) = self.next_part_number_marker {
            s.content("NextPartNumberMarker", val)?;
        }
        if let Some(ref val) = self.owner {
            s.content("Owner", val)?;
        }
        if let Some(ref val) = self.part_number_marker {
            s.content("PartNumberMarker", val)?;
        }
        if let Some(iter) = &self.parts {
            s.flattened_list("Part", iter)?;
        }
        if let Some(ref val) = self.storage_class {
            s.content("StorageClass", val)?;
        }
        if let Some(ref val) = self.upload_id {
            s.content("UploadId", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for LoggingEnabled {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("TargetBucket", &self.target_bucket)?;
        if let Some(iter) = &self.target_grants {
            s.list("TargetGrants", "Grant", iter)?;
        }
        s.content("TargetPrefix", &self.target_prefix)?;
        Ok(())
    }
}

impl SerializeContent for MFADeleteStatus {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for Metrics {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.event_threshold {
            s.content("EventThreshold", val)?;
        }
        s.content("Status", &self.status)?;
        Ok(())
    }
}

impl SerializeContent for MetricsAndOperator {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.access_point_arn {
            s.content("AccessPointArn", val)?;
        }
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        if let Some(iter) = &self.tags {
            s.flattened_list("Tag", iter)?;
        }
        Ok(())
    }
}

impl SerializeContent for MetricsConfiguration {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.filter {
            s.content("Filter", val)?;
        }
        s.content("Id", &self.id)?;
        Ok(())
    }
}

impl SerializeContent for MetricsFilter {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        match self {
            Self::AccessPointArn(x) => s.content("AccessPointArn", x),
            Self::And(x) => s.content("And", x),
            Self::Prefix(x) => s.content("Prefix", x),
            Self::Tag(x) => s.content("Tag", x),
        }
    }
}

impl SerializeContent for MetricsStatus {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for MultipartUpload {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.checksum_algorithm {
            s.content("ChecksumAlgorithm", val)?;
        }
        if let Some(ref val) = self.initiated {
            s.timestamp("Initiated", val, TimestampFormat::DateTime)?;
        }
        if let Some(ref val) = self.initiator {
            s.content("Initiator", val)?;
        }
        if let Some(ref val) = self.key {
            s.content("Key", val)?;
        }
        if let Some(ref val) = self.owner {
            s.content("Owner", val)?;
        }
        if let Some(ref val) = self.storage_class {
            s.content("StorageClass", val)?;
        }
        if let Some(ref val) = self.upload_id {
            s.content("UploadId", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for NoncurrentVersionExpiration {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("NewerNoncurrentVersions", &self.newer_noncurrent_versions)?;
        s.content("NoncurrentDays", &self.noncurrent_days)?;
        Ok(())
    }
}

impl SerializeContent for NoncurrentVersionTransition {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("NewerNoncurrentVersions", &self.newer_noncurrent_versions)?;
        s.content("NoncurrentDays", &self.noncurrent_days)?;
        if let Some(ref val) = self.storage_class {
            s.content("StorageClass", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for NotificationConfigurationFilter {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.key {
            s.content("S3Key", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for Object {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(iter) = &self.checksum_algorithm {
            s.flattened_list("ChecksumAlgorithm", iter)?;
        }
        if let Some(ref val) = self.e_tag {
            s.content("ETag", val)?;
        }
        if let Some(ref val) = self.key {
            s.content("Key", val)?;
        }
        if let Some(ref val) = self.last_modified {
            s.timestamp("LastModified", val, TimestampFormat::DateTime)?;
        }
        if let Some(ref val) = self.owner {
            s.content("Owner", val)?;
        }
        s.content("Size", &self.size)?;
        if let Some(ref val) = self.storage_class {
            s.content("StorageClass", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for ObjectLockConfiguration {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.object_lock_enabled {
            s.content("ObjectLockEnabled", val)?;
        }
        if let Some(ref val) = self.rule {
            s.content("Rule", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for ObjectLockEnabled {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for ObjectLockLegalHold {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.status {
            s.content("Status", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for ObjectLockLegalHoldStatus {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for ObjectLockRetention {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.mode {
            s.content("Mode", val)?;
        }
        if let Some(ref val) = self.retain_until_date {
            s.timestamp("RetainUntilDate", val, TimestampFormat::DateTime)?;
        }
        Ok(())
    }
}

impl SerializeContent for ObjectLockRetentionMode {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for ObjectLockRule {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.default_retention {
            s.content("DefaultRetention", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for ObjectOwnership {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for ObjectPart {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.checksum_crc32 {
            s.content("ChecksumCRC32", val)?;
        }
        if let Some(ref val) = self.checksum_crc32c {
            s.content("ChecksumCRC32C", val)?;
        }
        if let Some(ref val) = self.checksum_sha1 {
            s.content("ChecksumSHA1", val)?;
        }
        if let Some(ref val) = self.checksum_sha256 {
            s.content("ChecksumSHA256", val)?;
        }
        s.content("PartNumber", &self.part_number)?;
        s.content("Size", &self.size)?;
        Ok(())
    }
}

impl SerializeContent for ObjectStorageClass {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for ObjectVersion {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(iter) = &self.checksum_algorithm {
            s.flattened_list("ChecksumAlgorithm", iter)?;
        }
        if let Some(ref val) = self.e_tag {
            s.content("ETag", val)?;
        }
        s.content("IsLatest", &self.is_latest)?;
        if let Some(ref val) = self.key {
            s.content("Key", val)?;
        }
        if let Some(ref val) = self.last_modified {
            s.timestamp("LastModified", val, TimestampFormat::DateTime)?;
        }
        if let Some(ref val) = self.owner {
            s.content("Owner", val)?;
        }
        s.content("Size", &self.size)?;
        if let Some(ref val) = self.storage_class {
            s.content("StorageClass", val)?;
        }
        if let Some(ref val) = self.version_id {
            s.content("VersionId", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for ObjectVersionStorageClass {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for Owner {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.display_name {
            s.content("DisplayName", val)?;
        }
        if let Some(ref val) = self.id {
            s.content("ID", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for OwnerOverride {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for OwnershipControls {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        {
            let iter = &self.rules;
            s.flattened_list("Rule", iter)?;
        }
        Ok(())
    }
}

impl SerializeContent for OwnershipControlsRule {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("ObjectOwnership", &self.object_ownership)?;
        Ok(())
    }
}

impl SerializeContent for Part {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.checksum_crc32 {
            s.content("ChecksumCRC32", val)?;
        }
        if let Some(ref val) = self.checksum_crc32c {
            s.content("ChecksumCRC32C", val)?;
        }
        if let Some(ref val) = self.checksum_sha1 {
            s.content("ChecksumSHA1", val)?;
        }
        if let Some(ref val) = self.checksum_sha256 {
            s.content("ChecksumSHA256", val)?;
        }
        if let Some(ref val) = self.e_tag {
            s.content("ETag", val)?;
        }
        if let Some(ref val) = self.last_modified {
            s.timestamp("LastModified", val, TimestampFormat::DateTime)?;
        }
        s.content("PartNumber", &self.part_number)?;
        s.content("Size", &self.size)?;
        Ok(())
    }
}

impl SerializeContent for Payer {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for Permission {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for PolicyStatus {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("IsPublic", &self.is_public)?;
        Ok(())
    }
}

impl SerializeContent for Progress {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("BytesProcessed", &self.bytes_processed)?;
        s.content("BytesReturned", &self.bytes_returned)?;
        s.content("BytesScanned", &self.bytes_scanned)?;
        Ok(())
    }
}

impl SerializeContent for Protocol {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for PublicAccessBlockConfiguration {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("BlockPublicAcls", &self.block_public_acls)?;
        s.content("BlockPublicPolicy", &self.block_public_policy)?;
        s.content("IgnorePublicAcls", &self.ignore_public_acls)?;
        s.content("RestrictPublicBuckets", &self.restrict_public_buckets)?;
        Ok(())
    }
}

impl SerializeContent for QueueConfiguration {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        {
            let iter = &self.events;
            s.flattened_list("Event", iter)?;
        }
        if let Some(ref val) = self.filter {
            s.content("Filter", val)?;
        }
        if let Some(ref val) = self.id {
            s.content("Id", val)?;
        }
        s.content("Queue", &self.queue_arn)?;
        Ok(())
    }
}

impl SerializeContent for Redirect {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.host_name {
            s.content("HostName", val)?;
        }
        if let Some(ref val) = self.http_redirect_code {
            s.content("HttpRedirectCode", val)?;
        }
        if let Some(ref val) = self.protocol {
            s.content("Protocol", val)?;
        }
        if let Some(ref val) = self.replace_key_prefix_with {
            s.content("ReplaceKeyPrefixWith", val)?;
        }
        if let Some(ref val) = self.replace_key_with {
            s.content("ReplaceKeyWith", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for RedirectAllRequestsTo {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("HostName", &self.host_name)?;
        if let Some(ref val) = self.protocol {
            s.content("Protocol", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for ReplicaModifications {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("Status", &self.status)?;
        Ok(())
    }
}

impl SerializeContent for ReplicaModificationsStatus {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for ReplicationConfiguration {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("Role", &self.role)?;
        {
            let iter = &self.rules;
            s.flattened_list("Rule", iter)?;
        }
        Ok(())
    }
}

impl SerializeContent for ReplicationRule {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.delete_marker_replication {
            s.content("DeleteMarkerReplication", val)?;
        }
        s.content("Destination", &self.destination)?;
        if let Some(ref val) = self.existing_object_replication {
            s.content("ExistingObjectReplication", val)?;
        }
        if let Some(ref val) = self.filter {
            s.content("Filter", val)?;
        }
        if let Some(ref val) = self.id {
            s.content("ID", val)?;
        }
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        s.content("Priority", &self.priority)?;
        if let Some(ref val) = self.source_selection_criteria {
            s.content("SourceSelectionCriteria", val)?;
        }
        s.content("Status", &self.status)?;
        Ok(())
    }
}

impl SerializeContent for ReplicationRuleAndOperator {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.prefix {
            s.content("Prefix", val)?;
        }
        if let Some(iter) = &self.tags {
            s.flattened_list("Tag", iter)?;
        }
        Ok(())
    }
}

impl SerializeContent for ReplicationRuleFilter {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        match self {
            Self::And(x) => s.content("And", x),
            Self::Prefix(x) => s.content("Prefix", x),
            Self::Tag(x) => s.content("Tag", x),
        }
    }
}

impl SerializeContent for ReplicationRuleStatus {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for ReplicationTime {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("Status", &self.status)?;
        s.content("Time", &self.time)?;
        Ok(())
    }
}

impl SerializeContent for ReplicationTimeStatus {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for ReplicationTimeValue {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("Minutes", &self.minutes)?;
        Ok(())
    }
}

impl SerializeContent for RoutingRule {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.condition {
            s.content("Condition", val)?;
        }
        s.content("Redirect", &self.redirect)?;
        Ok(())
    }
}

impl SerializeContent for S3KeyFilter {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(iter) = &self.filter_rules {
            s.flattened_list("FilterRule", iter)?;
        }
        Ok(())
    }
}

impl SerializeContent for SSEKMS {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("KeyId", &self.key_id)?;
        Ok(())
    }
}

impl SerializeContent for SSES3 {
    fn serialize_content<W: Write>(&self, _: &mut Serializer<W>) -> SerResult {
        Ok(())
    }
}

impl SerializeContent for ServerSideEncryption {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for ServerSideEncryptionByDefault {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.kms_master_key_id {
            s.content("KMSMasterKeyID", val)?;
        }
        s.content("SSEAlgorithm", &self.sse_algorithm)?;
        Ok(())
    }
}

impl SerializeContent for ServerSideEncryptionConfiguration {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        {
            let iter = &self.rules;
            s.flattened_list("Rule", iter)?;
        }
        Ok(())
    }
}

impl SerializeContent for ServerSideEncryptionRule {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.apply_server_side_encryption_by_default {
            s.content("ApplyServerSideEncryptionByDefault", val)?;
        }
        s.content("BucketKeyEnabled", &self.bucket_key_enabled)?;
        Ok(())
    }
}

impl SerializeContent for SourceSelectionCriteria {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.replica_modifications {
            s.content("ReplicaModifications", val)?;
        }
        if let Some(ref val) = self.sse_kms_encrypted_objects {
            s.content("SseKmsEncryptedObjects", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for SseKmsEncryptedObjects {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("Status", &self.status)?;
        Ok(())
    }
}

impl SerializeContent for SseKmsEncryptedObjectsStatus {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for Stats {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("BytesProcessed", &self.bytes_processed)?;
        s.content("BytesReturned", &self.bytes_returned)?;
        s.content("BytesScanned", &self.bytes_scanned)?;
        Ok(())
    }
}

impl SerializeContent for StorageClass {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for StorageClassAnalysis {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.data_export {
            s.content("DataExport", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for StorageClassAnalysisDataExport {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("Destination", &self.destination)?;
        s.content("OutputSchemaVersion", &self.output_schema_version)?;
        Ok(())
    }
}

impl SerializeContent for StorageClassAnalysisSchemaVersion {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for Tag {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("Key", &self.key)?;
        s.content("Value", &self.value)?;
        Ok(())
    }
}

impl SerializeContent for TargetGrant {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.grantee {
            s.content("Grantee", val)?;
        }
        if let Some(ref val) = self.permission {
            s.content("Permission", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for Tiering {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("AccessTier", &self.access_tier)?;
        s.content("Days", &self.days)?;
        Ok(())
    }
}

impl SerializeContent for TopicConfiguration {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        {
            let iter = &self.events;
            s.flattened_list("Event", iter)?;
        }
        if let Some(ref val) = self.filter {
            s.content("Filter", val)?;
        }
        if let Some(ref val) = self.id {
            s.content("Id", val)?;
        }
        s.content("Topic", &self.topic_arn)?;
        Ok(())
    }
}

impl SerializeContent for Transition {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        if let Some(ref val) = self.date {
            s.timestamp("Date", val, TimestampFormat::DateTime)?;
        }
        s.content("Days", &self.days)?;
        if let Some(ref val) = self.storage_class {
            s.content("StorageClass", val)?;
        }
        Ok(())
    }
}

impl SerializeContent for TransitionStorageClass {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl SerializeContent for Type {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_str().serialize_content(s)
    }
}

impl Serialize for AnalyticsConfiguration {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("AnalyticsConfiguration", self)
    }
}

impl Serialize for CompleteMultipartUploadOutput {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("CompleteMultipartUploadResult", self)
    }
}

impl Serialize for CopyObjectResult {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("CopyObjectResult", self)
    }
}

impl Serialize for CopyPartResult {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("CopyPartResult", self)
    }
}

impl Serialize for CreateMultipartUploadOutput {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("InitiateMultipartUploadResult", self)
    }
}

impl Serialize for DeleteObjectsOutput {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("DeleteResult", self)
    }
}

impl Serialize for GetBucketAccelerateConfigurationOutput {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("AccelerateConfiguration", self)
    }
}

impl Serialize for GetBucketAclOutput {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("AccessControlPolicy", self)
    }
}

impl Serialize for GetBucketCorsOutput {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("CORSConfiguration", self)
    }
}

impl Serialize for GetBucketLifecycleConfigurationOutput {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("LifecycleConfiguration", self)
    }
}

impl Serialize for GetBucketLocationOutput {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("LocationConstraint", self)
    }
}

impl Serialize for GetBucketLoggingOutput {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("BucketLoggingStatus", self)
    }
}

impl Serialize for GetBucketNotificationConfigurationOutput {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("GetBucketNotificationConfigurationOutput", self)
    }
}

impl Serialize for GetBucketRequestPaymentOutput {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("RequestPaymentConfiguration", self)
    }
}

impl Serialize for GetBucketTaggingOutput {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("Tagging", self)
    }
}

impl Serialize for GetBucketVersioningOutput {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("VersioningConfiguration", self)
    }
}

impl Serialize for GetBucketWebsiteOutput {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("WebsiteConfiguration", self)
    }
}

impl Serialize for GetObjectAclOutput {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("AccessControlPolicy", self)
    }
}

impl Serialize for GetObjectAttributesOutput {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("GetObjectAttributesOutput", self)
    }
}

impl Serialize for GetObjectTaggingOutput {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("Tagging", self)
    }
}

impl Serialize for IntelligentTieringConfiguration {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("IntelligentTieringConfiguration", self)
    }
}

impl Serialize for InventoryConfiguration {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("InventoryConfiguration", self)
    }
}

impl Serialize for ListBucketAnalyticsConfigurationsOutput {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("ListBucketAnalyticsConfigurationResult", self)
    }
}

impl Serialize for ListBucketIntelligentTieringConfigurationsOutput {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("ListBucketIntelligentTieringConfigurationsOutput", self)
    }
}

impl Serialize for ListBucketInventoryConfigurationsOutput {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("ListInventoryConfigurationsResult", self)
    }
}

impl Serialize for ListBucketMetricsConfigurationsOutput {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("ListMetricsConfigurationsResult", self)
    }
}

impl Serialize for ListBucketsOutput {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("ListAllMyBucketsResult", self)
    }
}

impl Serialize for ListMultipartUploadsOutput {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("ListMultipartUploadsResult", self)
    }
}

impl Serialize for ListObjectVersionsOutput {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("ListVersionsResult", self)
    }
}

impl Serialize for ListObjectsOutput {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("ListBucketResult", self)
    }
}

impl Serialize for ListObjectsV2Output {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("ListBucketResult", self)
    }
}

impl Serialize for ListPartsOutput {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("ListPartsResult", self)
    }
}

impl Serialize for MetricsConfiguration {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("MetricsConfiguration", self)
    }
}

impl Serialize for ObjectLockConfiguration {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("ObjectLockConfiguration", self)
    }
}

impl Serialize for ObjectLockLegalHold {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("ObjectLockLegalHold", self)
    }
}

impl Serialize for ObjectLockRetention {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("ObjectLockRetention", self)
    }
}

impl Serialize for OwnershipControls {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("OwnershipControls", self)
    }
}

impl Serialize for PolicyStatus {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("PolicyStatus", self)
    }
}

impl Serialize for Progress {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("Progress", self)
    }
}

impl Serialize for PublicAccessBlockConfiguration {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("PublicAccessBlockConfiguration", self)
    }
}

impl Serialize for ReplicationConfiguration {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("ReplicationConfiguration", self)
    }
}

impl Serialize for ServerSideEncryptionConfiguration {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("ServerSideEncryptionConfiguration", self)
    }
}

impl Serialize for Stats {
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.content("Stats", self)
    }
}

impl<'xml> DeserializeContent<'xml> for AbortIncompleteMultipartUpload {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut days_after_initiation: Option<DaysAfterInitiation> = None;
        d.for_each_element(|d, x| match x {
            b"DaysAfterInitiation" => {
                if days_after_initiation.is_some() {
                    return Err(DeError::DuplicateField);
                }
                days_after_initiation = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            days_after_initiation: days_after_initiation.unwrap_or(0),
        })
    }
}

impl<'xml> DeserializeContent<'xml> for AccelerateConfiguration {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut status: Option<BucketAccelerateStatus> = None;
        d.for_each_element(|d, x| match x {
            b"Status" => {
                if status.is_some() {
                    return Err(DeError::DuplicateField);
                }
                status = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { status })
    }
}

impl<'xml> DeserializeContent<'xml> for AccessControlPolicy {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut grants: Option<Grants> = None;
        let mut owner: Option<Owner> = None;
        d.for_each_element(|d, x| match x {
            b"AccessControlList" => {
                if grants.is_some() {
                    return Err(DeError::DuplicateField);
                }
                grants = Some(d.list_content("member")?);
                Ok(())
            }
            b"Owner" => {
                if owner.is_some() {
                    return Err(DeError::DuplicateField);
                }
                owner = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { grants, owner })
    }
}

impl<'xml> DeserializeContent<'xml> for AccessControlTranslation {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut owner: Option<OwnerOverride> = None;
        d.for_each_element(|d, x| match x {
            b"Owner" => {
                if owner.is_some() {
                    return Err(DeError::DuplicateField);
                }
                owner = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            owner: owner.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for AnalyticsAndOperator {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut prefix: Option<Prefix> = None;
        let mut tags: Option<TagSet> = None;
        d.for_each_element(|d, x| match x {
            b"Prefix" => {
                if prefix.is_some() {
                    return Err(DeError::DuplicateField);
                }
                prefix = Some(d.content()?);
                Ok(())
            }
            b"Tag" => {
                let ans: Tag = d.content()?;
                tags.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { prefix, tags })
    }
}

impl<'xml> DeserializeContent<'xml> for AnalyticsConfiguration {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut filter: Option<AnalyticsFilter> = None;
        let mut id: Option<AnalyticsId> = None;
        let mut storage_class_analysis: Option<StorageClassAnalysis> = None;
        d.for_each_element(|d, x| match x {
            b"Filter" => {
                if filter.is_some() {
                    return Err(DeError::DuplicateField);
                }
                filter = Some(d.content()?);
                Ok(())
            }
            b"Id" => {
                if id.is_some() {
                    return Err(DeError::DuplicateField);
                }
                id = Some(d.content()?);
                Ok(())
            }
            b"StorageClassAnalysis" => {
                if storage_class_analysis.is_some() {
                    return Err(DeError::DuplicateField);
                }
                storage_class_analysis = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            filter,
            id: id.ok_or(DeError::MissingField)?,
            storage_class_analysis: storage_class_analysis.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for AnalyticsExportDestination {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut s3_bucket_destination: Option<AnalyticsS3BucketDestination> = None;
        d.for_each_element(|d, x| match x {
            b"S3BucketDestination" => {
                if s3_bucket_destination.is_some() {
                    return Err(DeError::DuplicateField);
                }
                s3_bucket_destination = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            s3_bucket_destination: s3_bucket_destination.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for AnalyticsFilter {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.element(|d, x| match x {
            b"And" => Ok(Self::And(d.content()?)),
            b"Prefix" => Ok(Self::Prefix(d.content()?)),
            b"Tag" => Ok(Self::Tag(d.content()?)),
            _ => Err(DeError::UnexpectedTagName),
        })
    }
}

impl<'xml> DeserializeContent<'xml> for AnalyticsS3BucketDestination {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut bucket: Option<BucketName> = None;
        let mut bucket_account_id: Option<AccountId> = None;
        let mut format: Option<AnalyticsS3ExportFileFormat> = None;
        let mut prefix: Option<Prefix> = None;
        d.for_each_element(|d, x| match x {
            b"Bucket" => {
                if bucket.is_some() {
                    return Err(DeError::DuplicateField);
                }
                bucket = Some(d.content()?);
                Ok(())
            }
            b"BucketAccountId" => {
                if bucket_account_id.is_some() {
                    return Err(DeError::DuplicateField);
                }
                bucket_account_id = Some(d.content()?);
                Ok(())
            }
            b"Format" => {
                if format.is_some() {
                    return Err(DeError::DuplicateField);
                }
                format = Some(d.content()?);
                Ok(())
            }
            b"Prefix" => {
                if prefix.is_some() {
                    return Err(DeError::DuplicateField);
                }
                prefix = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            bucket: bucket.ok_or(DeError::MissingField)?,
            bucket_account_id,
            format: format.ok_or(DeError::MissingField)?,
            prefix,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for AnalyticsS3ExportFileFormat {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for BucketAccelerateStatus {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for BucketLifecycleConfiguration {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut rules: Option<LifecycleRules> = None;
        d.for_each_element(|d, x| match x {
            b"Rule" => {
                let ans: LifecycleRule = d.content()?;
                rules.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            rules: rules.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for BucketLocationConstraint {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for BucketLoggingStatus {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut logging_enabled: Option<LoggingEnabled> = None;
        d.for_each_element(|d, x| match x {
            b"LoggingEnabled" => {
                if logging_enabled.is_some() {
                    return Err(DeError::DuplicateField);
                }
                logging_enabled = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { logging_enabled })
    }
}

impl<'xml> DeserializeContent<'xml> for BucketLogsPermission {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for BucketVersioningStatus {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for CORSConfiguration {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut cors_rules: Option<CORSRules> = None;
        d.for_each_element(|d, x| match x {
            b"CORSRule" => {
                let ans: CORSRule = d.content()?;
                cors_rules.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            cors_rules: cors_rules.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for CORSRule {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut allowed_headers: Option<AllowedHeaders> = None;
        let mut allowed_methods: Option<AllowedMethods> = None;
        let mut allowed_origins: Option<AllowedOrigins> = None;
        let mut expose_headers: Option<ExposeHeaders> = None;
        let mut id: Option<ID> = None;
        let mut max_age_seconds: Option<MaxAgeSeconds> = None;
        d.for_each_element(|d, x| match x {
            b"AllowedHeader" => {
                let ans: AllowedHeader = d.content()?;
                allowed_headers.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            b"AllowedMethod" => {
                let ans: AllowedMethod = d.content()?;
                allowed_methods.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            b"AllowedOrigin" => {
                let ans: AllowedOrigin = d.content()?;
                allowed_origins.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            b"ExposeHeader" => {
                let ans: ExposeHeader = d.content()?;
                expose_headers.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            b"ID" => {
                if id.is_some() {
                    return Err(DeError::DuplicateField);
                }
                id = Some(d.content()?);
                Ok(())
            }
            b"MaxAgeSeconds" => {
                if max_age_seconds.is_some() {
                    return Err(DeError::DuplicateField);
                }
                max_age_seconds = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            allowed_headers,
            allowed_methods: allowed_methods.ok_or(DeError::MissingField)?,
            allowed_origins: allowed_origins.ok_or(DeError::MissingField)?,
            expose_headers,
            id,
            max_age_seconds: max_age_seconds.unwrap_or(0),
        })
    }
}

impl<'xml> DeserializeContent<'xml> for CSVInput {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut allow_quoted_record_delimiter: Option<AllowQuotedRecordDelimiter> = None;
        let mut comments: Option<Comments> = None;
        let mut field_delimiter: Option<FieldDelimiter> = None;
        let mut file_header_info: Option<FileHeaderInfo> = None;
        let mut quote_character: Option<QuoteCharacter> = None;
        let mut quote_escape_character: Option<QuoteEscapeCharacter> = None;
        let mut record_delimiter: Option<RecordDelimiter> = None;
        d.for_each_element(|d, x| match x {
            b"AllowQuotedRecordDelimiter" => {
                if allow_quoted_record_delimiter.is_some() {
                    return Err(DeError::DuplicateField);
                }
                allow_quoted_record_delimiter = Some(d.content()?);
                Ok(())
            }
            b"Comments" => {
                if comments.is_some() {
                    return Err(DeError::DuplicateField);
                }
                comments = Some(d.content()?);
                Ok(())
            }
            b"FieldDelimiter" => {
                if field_delimiter.is_some() {
                    return Err(DeError::DuplicateField);
                }
                field_delimiter = Some(d.content()?);
                Ok(())
            }
            b"FileHeaderInfo" => {
                if file_header_info.is_some() {
                    return Err(DeError::DuplicateField);
                }
                file_header_info = Some(d.content()?);
                Ok(())
            }
            b"QuoteCharacter" => {
                if quote_character.is_some() {
                    return Err(DeError::DuplicateField);
                }
                quote_character = Some(d.content()?);
                Ok(())
            }
            b"QuoteEscapeCharacter" => {
                if quote_escape_character.is_some() {
                    return Err(DeError::DuplicateField);
                }
                quote_escape_character = Some(d.content()?);
                Ok(())
            }
            b"RecordDelimiter" => {
                if record_delimiter.is_some() {
                    return Err(DeError::DuplicateField);
                }
                record_delimiter = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            allow_quoted_record_delimiter: allow_quoted_record_delimiter.unwrap_or(false),
            comments,
            field_delimiter,
            file_header_info,
            quote_character,
            quote_escape_character,
            record_delimiter,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for CSVOutput {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut field_delimiter: Option<FieldDelimiter> = None;
        let mut quote_character: Option<QuoteCharacter> = None;
        let mut quote_escape_character: Option<QuoteEscapeCharacter> = None;
        let mut quote_fields: Option<QuoteFields> = None;
        let mut record_delimiter: Option<RecordDelimiter> = None;
        d.for_each_element(|d, x| match x {
            b"FieldDelimiter" => {
                if field_delimiter.is_some() {
                    return Err(DeError::DuplicateField);
                }
                field_delimiter = Some(d.content()?);
                Ok(())
            }
            b"QuoteCharacter" => {
                if quote_character.is_some() {
                    return Err(DeError::DuplicateField);
                }
                quote_character = Some(d.content()?);
                Ok(())
            }
            b"QuoteEscapeCharacter" => {
                if quote_escape_character.is_some() {
                    return Err(DeError::DuplicateField);
                }
                quote_escape_character = Some(d.content()?);
                Ok(())
            }
            b"QuoteFields" => {
                if quote_fields.is_some() {
                    return Err(DeError::DuplicateField);
                }
                quote_fields = Some(d.content()?);
                Ok(())
            }
            b"RecordDelimiter" => {
                if record_delimiter.is_some() {
                    return Err(DeError::DuplicateField);
                }
                record_delimiter = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            field_delimiter,
            quote_character,
            quote_escape_character,
            quote_fields,
            record_delimiter,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for CompletedMultipartUpload {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut parts: Option<CompletedPartList> = None;
        d.for_each_element(|d, x| match x {
            b"Part" => {
                let ans: CompletedPart = d.content()?;
                parts.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { parts })
    }
}

impl<'xml> DeserializeContent<'xml> for CompletedPart {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut checksum_crc32: Option<ChecksumCRC32> = None;
        let mut checksum_crc32c: Option<ChecksumCRC32C> = None;
        let mut checksum_sha1: Option<ChecksumSHA1> = None;
        let mut checksum_sha256: Option<ChecksumSHA256> = None;
        let mut e_tag: Option<ETag> = None;
        let mut part_number: Option<PartNumber> = None;
        d.for_each_element(|d, x| match x {
            b"ChecksumCRC32" => {
                if checksum_crc32.is_some() {
                    return Err(DeError::DuplicateField);
                }
                checksum_crc32 = Some(d.content()?);
                Ok(())
            }
            b"ChecksumCRC32C" => {
                if checksum_crc32c.is_some() {
                    return Err(DeError::DuplicateField);
                }
                checksum_crc32c = Some(d.content()?);
                Ok(())
            }
            b"ChecksumSHA1" => {
                if checksum_sha1.is_some() {
                    return Err(DeError::DuplicateField);
                }
                checksum_sha1 = Some(d.content()?);
                Ok(())
            }
            b"ChecksumSHA256" => {
                if checksum_sha256.is_some() {
                    return Err(DeError::DuplicateField);
                }
                checksum_sha256 = Some(d.content()?);
                Ok(())
            }
            b"ETag" => {
                if e_tag.is_some() {
                    return Err(DeError::DuplicateField);
                }
                e_tag = Some(d.content()?);
                Ok(())
            }
            b"PartNumber" => {
                if part_number.is_some() {
                    return Err(DeError::DuplicateField);
                }
                part_number = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            checksum_crc32,
            checksum_crc32c,
            checksum_sha1,
            checksum_sha256,
            e_tag,
            part_number: part_number.unwrap_or(0),
        })
    }
}

impl<'xml> DeserializeContent<'xml> for CompressionType {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for Condition {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut http_error_code_returned_equals: Option<HttpErrorCodeReturnedEquals> = None;
        let mut key_prefix_equals: Option<KeyPrefixEquals> = None;
        d.for_each_element(|d, x| match x {
            b"HttpErrorCodeReturnedEquals" => {
                if http_error_code_returned_equals.is_some() {
                    return Err(DeError::DuplicateField);
                }
                http_error_code_returned_equals = Some(d.content()?);
                Ok(())
            }
            b"KeyPrefixEquals" => {
                if key_prefix_equals.is_some() {
                    return Err(DeError::DuplicateField);
                }
                key_prefix_equals = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            http_error_code_returned_equals,
            key_prefix_equals,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for CreateBucketConfiguration {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut location_constraint: Option<BucketLocationConstraint> = None;
        d.for_each_element(|d, x| match x {
            b"LocationConstraint" => {
                if location_constraint.is_some() {
                    return Err(DeError::DuplicateField);
                }
                location_constraint = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { location_constraint })
    }
}

impl<'xml> DeserializeContent<'xml> for DefaultRetention {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut days: Option<Days> = None;
        let mut mode: Option<ObjectLockRetentionMode> = None;
        let mut years: Option<Years> = None;
        d.for_each_element(|d, x| match x {
            b"Days" => {
                if days.is_some() {
                    return Err(DeError::DuplicateField);
                }
                days = Some(d.content()?);
                Ok(())
            }
            b"Mode" => {
                if mode.is_some() {
                    return Err(DeError::DuplicateField);
                }
                mode = Some(d.content()?);
                Ok(())
            }
            b"Years" => {
                if years.is_some() {
                    return Err(DeError::DuplicateField);
                }
                years = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            days: days.unwrap_or(0),
            mode,
            years: years.unwrap_or(0),
        })
    }
}

impl<'xml> DeserializeContent<'xml> for Delete {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut objects: Option<ObjectIdentifierList> = None;
        let mut quiet: Option<Quiet> = None;
        d.for_each_element(|d, x| match x {
            b"Object" => {
                let ans: ObjectIdentifier = d.content()?;
                objects.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            b"Quiet" => {
                if quiet.is_some() {
                    return Err(DeError::DuplicateField);
                }
                quiet = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            objects: objects.ok_or(DeError::MissingField)?,
            quiet: quiet.unwrap_or(false),
        })
    }
}

impl<'xml> DeserializeContent<'xml> for DeleteMarkerReplication {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut status: Option<DeleteMarkerReplicationStatus> = None;
        d.for_each_element(|d, x| match x {
            b"Status" => {
                if status.is_some() {
                    return Err(DeError::DuplicateField);
                }
                status = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { status })
    }
}

impl<'xml> DeserializeContent<'xml> for DeleteMarkerReplicationStatus {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for Destination {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut access_control_translation: Option<AccessControlTranslation> = None;
        let mut account: Option<AccountId> = None;
        let mut bucket: Option<BucketName> = None;
        let mut encryption_configuration: Option<EncryptionConfiguration> = None;
        let mut metrics: Option<Metrics> = None;
        let mut replication_time: Option<ReplicationTime> = None;
        let mut storage_class: Option<StorageClass> = None;
        d.for_each_element(|d, x| match x {
            b"AccessControlTranslation" => {
                if access_control_translation.is_some() {
                    return Err(DeError::DuplicateField);
                }
                access_control_translation = Some(d.content()?);
                Ok(())
            }
            b"Account" => {
                if account.is_some() {
                    return Err(DeError::DuplicateField);
                }
                account = Some(d.content()?);
                Ok(())
            }
            b"Bucket" => {
                if bucket.is_some() {
                    return Err(DeError::DuplicateField);
                }
                bucket = Some(d.content()?);
                Ok(())
            }
            b"EncryptionConfiguration" => {
                if encryption_configuration.is_some() {
                    return Err(DeError::DuplicateField);
                }
                encryption_configuration = Some(d.content()?);
                Ok(())
            }
            b"Metrics" => {
                if metrics.is_some() {
                    return Err(DeError::DuplicateField);
                }
                metrics = Some(d.content()?);
                Ok(())
            }
            b"ReplicationTime" => {
                if replication_time.is_some() {
                    return Err(DeError::DuplicateField);
                }
                replication_time = Some(d.content()?);
                Ok(())
            }
            b"StorageClass" => {
                if storage_class.is_some() {
                    return Err(DeError::DuplicateField);
                }
                storage_class = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            access_control_translation,
            account,
            bucket: bucket.ok_or(DeError::MissingField)?,
            encryption_configuration,
            metrics,
            replication_time,
            storage_class,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for Encryption {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut encryption_type: Option<ServerSideEncryption> = None;
        let mut kms_context: Option<KMSContext> = None;
        let mut kms_key_id: Option<SSEKMSKeyId> = None;
        d.for_each_element(|d, x| match x {
            b"EncryptionType" => {
                if encryption_type.is_some() {
                    return Err(DeError::DuplicateField);
                }
                encryption_type = Some(d.content()?);
                Ok(())
            }
            b"KMSContext" => {
                if kms_context.is_some() {
                    return Err(DeError::DuplicateField);
                }
                kms_context = Some(d.content()?);
                Ok(())
            }
            b"KMSKeyId" => {
                if kms_key_id.is_some() {
                    return Err(DeError::DuplicateField);
                }
                kms_key_id = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            encryption_type: encryption_type.ok_or(DeError::MissingField)?,
            kms_context,
            kms_key_id,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for EncryptionConfiguration {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut replica_kms_key_id: Option<ReplicaKmsKeyID> = None;
        d.for_each_element(|d, x| match x {
            b"ReplicaKmsKeyID" => {
                if replica_kms_key_id.is_some() {
                    return Err(DeError::DuplicateField);
                }
                replica_kms_key_id = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { replica_kms_key_id })
    }
}

impl<'xml> DeserializeContent<'xml> for ErrorDocument {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut key: Option<ObjectKey> = None;
        d.for_each_element(|d, x| match x {
            b"Key" => {
                if key.is_some() {
                    return Err(DeError::DuplicateField);
                }
                key = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            key: key.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for EventBridgeConfiguration {
    fn deserialize_content(_: &mut Deserializer<'xml>) -> DeResult<Self> {
        Ok(Self {})
    }
}

impl<'xml> DeserializeContent<'xml> for ExistingObjectReplication {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut status: Option<ExistingObjectReplicationStatus> = None;
        d.for_each_element(|d, x| match x {
            b"Status" => {
                if status.is_some() {
                    return Err(DeError::DuplicateField);
                }
                status = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            status: status.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for ExistingObjectReplicationStatus {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for ExpirationStatus {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for ExpressionType {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for FileHeaderInfo {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for FilterRule {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut name: Option<FilterRuleName> = None;
        let mut value: Option<FilterRuleValue> = None;
        d.for_each_element(|d, x| match x {
            b"Name" => {
                if name.is_some() {
                    return Err(DeError::DuplicateField);
                }
                name = Some(d.content()?);
                Ok(())
            }
            b"Value" => {
                if value.is_some() {
                    return Err(DeError::DuplicateField);
                }
                value = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { name, value })
    }
}

impl<'xml> DeserializeContent<'xml> for FilterRuleName {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for GlacierJobParameters {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut tier: Option<Tier> = None;
        d.for_each_element(|d, x| match x {
            b"Tier" => {
                if tier.is_some() {
                    return Err(DeError::DuplicateField);
                }
                tier = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            tier: tier.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for Grant {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut grantee: Option<Grantee> = None;
        let mut permission: Option<Permission> = None;
        d.for_each_element(|d, x| match x {
            b"Grantee" => {
                if grantee.is_some() {
                    return Err(DeError::DuplicateField);
                }
                grantee = Some(d.content()?);
                Ok(())
            }
            b"Permission" => {
                if permission.is_some() {
                    return Err(DeError::DuplicateField);
                }
                permission = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { grantee, permission })
    }
}

impl<'xml> DeserializeContent<'xml> for Grantee {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut display_name: Option<DisplayName> = None;
        let mut email_address: Option<EmailAddress> = None;
        let mut id: Option<ID> = None;
        let mut type_: Option<Type> = None;
        let mut uri: Option<URI> = None;
        d.for_each_element(|d, x| match x {
            b"DisplayName" => {
                if display_name.is_some() {
                    return Err(DeError::DuplicateField);
                }
                display_name = Some(d.content()?);
                Ok(())
            }
            b"EmailAddress" => {
                if email_address.is_some() {
                    return Err(DeError::DuplicateField);
                }
                email_address = Some(d.content()?);
                Ok(())
            }
            b"ID" => {
                if id.is_some() {
                    return Err(DeError::DuplicateField);
                }
                id = Some(d.content()?);
                Ok(())
            }
            b"xsi:type" => {
                if type_.is_some() {
                    return Err(DeError::DuplicateField);
                }
                type_ = Some(d.content()?);
                Ok(())
            }
            b"URI" => {
                if uri.is_some() {
                    return Err(DeError::DuplicateField);
                }
                uri = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            display_name,
            email_address,
            id,
            type_: type_.ok_or(DeError::MissingField)?,
            uri,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for IndexDocument {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut suffix: Option<Suffix> = None;
        d.for_each_element(|d, x| match x {
            b"Suffix" => {
                if suffix.is_some() {
                    return Err(DeError::DuplicateField);
                }
                suffix = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            suffix: suffix.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for InputSerialization {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut csv: Option<CSVInput> = None;
        let mut compression_type: Option<CompressionType> = None;
        let mut json: Option<JSONInput> = None;
        let mut parquet: Option<ParquetInput> = None;
        d.for_each_element(|d, x| match x {
            b"CSV" => {
                if csv.is_some() {
                    return Err(DeError::DuplicateField);
                }
                csv = Some(d.content()?);
                Ok(())
            }
            b"CompressionType" => {
                if compression_type.is_some() {
                    return Err(DeError::DuplicateField);
                }
                compression_type = Some(d.content()?);
                Ok(())
            }
            b"JSON" => {
                if json.is_some() {
                    return Err(DeError::DuplicateField);
                }
                json = Some(d.content()?);
                Ok(())
            }
            b"Parquet" => {
                if parquet.is_some() {
                    return Err(DeError::DuplicateField);
                }
                parquet = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            csv,
            compression_type,
            json,
            parquet,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for IntelligentTieringAccessTier {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for IntelligentTieringAndOperator {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut prefix: Option<Prefix> = None;
        let mut tags: Option<TagSet> = None;
        d.for_each_element(|d, x| match x {
            b"Prefix" => {
                if prefix.is_some() {
                    return Err(DeError::DuplicateField);
                }
                prefix = Some(d.content()?);
                Ok(())
            }
            b"Tag" => {
                let ans: Tag = d.content()?;
                tags.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { prefix, tags })
    }
}

impl<'xml> DeserializeContent<'xml> for IntelligentTieringConfiguration {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut filter: Option<IntelligentTieringFilter> = None;
        let mut id: Option<IntelligentTieringId> = None;
        let mut status: Option<IntelligentTieringStatus> = None;
        let mut tierings: Option<TieringList> = None;
        d.for_each_element(|d, x| match x {
            b"Filter" => {
                if filter.is_some() {
                    return Err(DeError::DuplicateField);
                }
                filter = Some(d.content()?);
                Ok(())
            }
            b"Id" => {
                if id.is_some() {
                    return Err(DeError::DuplicateField);
                }
                id = Some(d.content()?);
                Ok(())
            }
            b"Status" => {
                if status.is_some() {
                    return Err(DeError::DuplicateField);
                }
                status = Some(d.content()?);
                Ok(())
            }
            b"Tiering" => {
                let ans: Tiering = d.content()?;
                tierings.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            filter,
            id: id.ok_or(DeError::MissingField)?,
            status: status.ok_or(DeError::MissingField)?,
            tierings: tierings.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for IntelligentTieringFilter {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut and: Option<IntelligentTieringAndOperator> = None;
        let mut prefix: Option<Prefix> = None;
        let mut tag: Option<Tag> = None;
        d.for_each_element(|d, x| match x {
            b"And" => {
                if and.is_some() {
                    return Err(DeError::DuplicateField);
                }
                and = Some(d.content()?);
                Ok(())
            }
            b"Prefix" => {
                if prefix.is_some() {
                    return Err(DeError::DuplicateField);
                }
                prefix = Some(d.content()?);
                Ok(())
            }
            b"Tag" => {
                if tag.is_some() {
                    return Err(DeError::DuplicateField);
                }
                tag = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { and, prefix, tag })
    }
}

impl<'xml> DeserializeContent<'xml> for IntelligentTieringStatus {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for InventoryConfiguration {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut destination: Option<InventoryDestination> = None;
        let mut filter: Option<InventoryFilter> = None;
        let mut id: Option<InventoryId> = None;
        let mut included_object_versions: Option<InventoryIncludedObjectVersions> = None;
        let mut is_enabled: Option<IsEnabled> = None;
        let mut optional_fields: Option<InventoryOptionalFields> = None;
        let mut schedule: Option<InventorySchedule> = None;
        d.for_each_element(|d, x| match x {
            b"Destination" => {
                if destination.is_some() {
                    return Err(DeError::DuplicateField);
                }
                destination = Some(d.content()?);
                Ok(())
            }
            b"Filter" => {
                if filter.is_some() {
                    return Err(DeError::DuplicateField);
                }
                filter = Some(d.content()?);
                Ok(())
            }
            b"Id" => {
                if id.is_some() {
                    return Err(DeError::DuplicateField);
                }
                id = Some(d.content()?);
                Ok(())
            }
            b"IncludedObjectVersions" => {
                if included_object_versions.is_some() {
                    return Err(DeError::DuplicateField);
                }
                included_object_versions = Some(d.content()?);
                Ok(())
            }
            b"IsEnabled" => {
                if is_enabled.is_some() {
                    return Err(DeError::DuplicateField);
                }
                is_enabled = Some(d.content()?);
                Ok(())
            }
            b"OptionalFields" => {
                if optional_fields.is_some() {
                    return Err(DeError::DuplicateField);
                }
                optional_fields = Some(d.list_content("member")?);
                Ok(())
            }
            b"Schedule" => {
                if schedule.is_some() {
                    return Err(DeError::DuplicateField);
                }
                schedule = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            destination: destination.ok_or(DeError::MissingField)?,
            filter,
            id: id.ok_or(DeError::MissingField)?,
            included_object_versions: included_object_versions.ok_or(DeError::MissingField)?,
            is_enabled: is_enabled.unwrap_or(false),
            optional_fields,
            schedule: schedule.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for InventoryDestination {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut s3_bucket_destination: Option<InventoryS3BucketDestination> = None;
        d.for_each_element(|d, x| match x {
            b"S3BucketDestination" => {
                if s3_bucket_destination.is_some() {
                    return Err(DeError::DuplicateField);
                }
                s3_bucket_destination = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            s3_bucket_destination: s3_bucket_destination.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for InventoryEncryption {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut ssekms: Option<SSEKMS> = None;
        let mut sses3: Option<SSES3> = None;
        d.for_each_element(|d, x| match x {
            b"SSE-KMS" => {
                if ssekms.is_some() {
                    return Err(DeError::DuplicateField);
                }
                ssekms = Some(d.content()?);
                Ok(())
            }
            b"SSE-S3" => {
                if sses3.is_some() {
                    return Err(DeError::DuplicateField);
                }
                sses3 = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { ssekms, sses3 })
    }
}

impl<'xml> DeserializeContent<'xml> for InventoryFilter {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut prefix: Option<Prefix> = None;
        d.for_each_element(|d, x| match x {
            b"Prefix" => {
                if prefix.is_some() {
                    return Err(DeError::DuplicateField);
                }
                prefix = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            prefix: prefix.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for InventoryFormat {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for InventoryFrequency {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for InventoryIncludedObjectVersions {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for InventoryOptionalField {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for InventoryS3BucketDestination {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut account_id: Option<AccountId> = None;
        let mut bucket: Option<BucketName> = None;
        let mut encryption: Option<InventoryEncryption> = None;
        let mut format: Option<InventoryFormat> = None;
        let mut prefix: Option<Prefix> = None;
        d.for_each_element(|d, x| match x {
            b"AccountId" => {
                if account_id.is_some() {
                    return Err(DeError::DuplicateField);
                }
                account_id = Some(d.content()?);
                Ok(())
            }
            b"Bucket" => {
                if bucket.is_some() {
                    return Err(DeError::DuplicateField);
                }
                bucket = Some(d.content()?);
                Ok(())
            }
            b"Encryption" => {
                if encryption.is_some() {
                    return Err(DeError::DuplicateField);
                }
                encryption = Some(d.content()?);
                Ok(())
            }
            b"Format" => {
                if format.is_some() {
                    return Err(DeError::DuplicateField);
                }
                format = Some(d.content()?);
                Ok(())
            }
            b"Prefix" => {
                if prefix.is_some() {
                    return Err(DeError::DuplicateField);
                }
                prefix = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            account_id,
            bucket: bucket.ok_or(DeError::MissingField)?,
            encryption,
            format: format.ok_or(DeError::MissingField)?,
            prefix,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for InventorySchedule {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut frequency: Option<InventoryFrequency> = None;
        d.for_each_element(|d, x| match x {
            b"Frequency" => {
                if frequency.is_some() {
                    return Err(DeError::DuplicateField);
                }
                frequency = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            frequency: frequency.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for JSONInput {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut type_: Option<JSONType> = None;
        d.for_each_element(|d, x| match x {
            b"Type" => {
                if type_.is_some() {
                    return Err(DeError::DuplicateField);
                }
                type_ = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { type_ })
    }
}

impl<'xml> DeserializeContent<'xml> for JSONOutput {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut record_delimiter: Option<RecordDelimiter> = None;
        d.for_each_element(|d, x| match x {
            b"RecordDelimiter" => {
                if record_delimiter.is_some() {
                    return Err(DeError::DuplicateField);
                }
                record_delimiter = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { record_delimiter })
    }
}

impl<'xml> DeserializeContent<'xml> for JSONType {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for LambdaFunctionConfiguration {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut events: Option<EventList> = None;
        let mut filter: Option<NotificationConfigurationFilter> = None;
        let mut id: Option<NotificationId> = None;
        let mut lambda_function_arn: Option<LambdaFunctionArn> = None;
        d.for_each_element(|d, x| match x {
            b"Event" => {
                let ans: Event = d.content()?;
                events.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            b"Filter" => {
                if filter.is_some() {
                    return Err(DeError::DuplicateField);
                }
                filter = Some(d.content()?);
                Ok(())
            }
            b"Id" => {
                if id.is_some() {
                    return Err(DeError::DuplicateField);
                }
                id = Some(d.content()?);
                Ok(())
            }
            b"CloudFunction" => {
                if lambda_function_arn.is_some() {
                    return Err(DeError::DuplicateField);
                }
                lambda_function_arn = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            events: events.ok_or(DeError::MissingField)?,
            filter,
            id,
            lambda_function_arn: lambda_function_arn.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for LifecycleExpiration {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut date: Option<Date> = None;
        let mut days: Option<Days> = None;
        let mut expired_object_delete_marker: Option<ExpiredObjectDeleteMarker> = None;
        d.for_each_element(|d, x| match x {
            b"Date" => {
                if date.is_some() {
                    return Err(DeError::DuplicateField);
                }
                date = Some(d.timestamp(TimestampFormat::DateTime)?);
                Ok(())
            }
            b"Days" => {
                if days.is_some() {
                    return Err(DeError::DuplicateField);
                }
                days = Some(d.content()?);
                Ok(())
            }
            b"ExpiredObjectDeleteMarker" => {
                if expired_object_delete_marker.is_some() {
                    return Err(DeError::DuplicateField);
                }
                expired_object_delete_marker = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            date,
            days: days.unwrap_or(0),
            expired_object_delete_marker: expired_object_delete_marker.unwrap_or(false),
        })
    }
}

impl<'xml> DeserializeContent<'xml> for LifecycleRule {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut abort_incomplete_multipart_upload: Option<AbortIncompleteMultipartUpload> = None;
        let mut expiration: Option<LifecycleExpiration> = None;
        let mut filter: Option<LifecycleRuleFilter> = None;
        let mut id: Option<ID> = None;
        let mut noncurrent_version_expiration: Option<NoncurrentVersionExpiration> = None;
        let mut noncurrent_version_transitions: Option<NoncurrentVersionTransitionList> = None;
        let mut prefix: Option<Prefix> = None;
        let mut status: Option<ExpirationStatus> = None;
        let mut transitions: Option<TransitionList> = None;
        d.for_each_element(|d, x| match x {
            b"AbortIncompleteMultipartUpload" => {
                if abort_incomplete_multipart_upload.is_some() {
                    return Err(DeError::DuplicateField);
                }
                abort_incomplete_multipart_upload = Some(d.content()?);
                Ok(())
            }
            b"Expiration" => {
                if expiration.is_some() {
                    return Err(DeError::DuplicateField);
                }
                expiration = Some(d.content()?);
                Ok(())
            }
            b"Filter" => {
                if filter.is_some() {
                    return Err(DeError::DuplicateField);
                }
                filter = Some(d.content()?);
                Ok(())
            }
            b"ID" => {
                if id.is_some() {
                    return Err(DeError::DuplicateField);
                }
                id = Some(d.content()?);
                Ok(())
            }
            b"NoncurrentVersionExpiration" => {
                if noncurrent_version_expiration.is_some() {
                    return Err(DeError::DuplicateField);
                }
                noncurrent_version_expiration = Some(d.content()?);
                Ok(())
            }
            b"NoncurrentVersionTransition" => {
                let ans: NoncurrentVersionTransition = d.content()?;
                noncurrent_version_transitions.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            b"Prefix" => {
                if prefix.is_some() {
                    return Err(DeError::DuplicateField);
                }
                prefix = Some(d.content()?);
                Ok(())
            }
            b"Status" => {
                if status.is_some() {
                    return Err(DeError::DuplicateField);
                }
                status = Some(d.content()?);
                Ok(())
            }
            b"Transition" => {
                let ans: Transition = d.content()?;
                transitions.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            abort_incomplete_multipart_upload,
            expiration,
            filter,
            id,
            noncurrent_version_expiration,
            noncurrent_version_transitions,
            prefix,
            status: status.ok_or(DeError::MissingField)?,
            transitions,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for LifecycleRuleAndOperator {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut object_size_greater_than: Option<ObjectSizeGreaterThanBytes> = None;
        let mut object_size_less_than: Option<ObjectSizeLessThanBytes> = None;
        let mut prefix: Option<Prefix> = None;
        let mut tags: Option<TagSet> = None;
        d.for_each_element(|d, x| match x {
            b"ObjectSizeGreaterThan" => {
                if object_size_greater_than.is_some() {
                    return Err(DeError::DuplicateField);
                }
                object_size_greater_than = Some(d.content()?);
                Ok(())
            }
            b"ObjectSizeLessThan" => {
                if object_size_less_than.is_some() {
                    return Err(DeError::DuplicateField);
                }
                object_size_less_than = Some(d.content()?);
                Ok(())
            }
            b"Prefix" => {
                if prefix.is_some() {
                    return Err(DeError::DuplicateField);
                }
                prefix = Some(d.content()?);
                Ok(())
            }
            b"Tag" => {
                let ans: Tag = d.content()?;
                tags.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            object_size_greater_than: object_size_greater_than.unwrap_or(0),
            object_size_less_than: object_size_less_than.unwrap_or(0),
            prefix,
            tags,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for LifecycleRuleFilter {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.element(|d, x| match x {
            b"And" => Ok(Self::And(d.content()?)),
            b"ObjectSizeGreaterThan" => Ok(Self::ObjectSizeGreaterThan(d.content()?)),
            b"ObjectSizeLessThan" => Ok(Self::ObjectSizeLessThan(d.content()?)),
            b"Prefix" => Ok(Self::Prefix(d.content()?)),
            b"Tag" => Ok(Self::Tag(d.content()?)),
            _ => Err(DeError::UnexpectedTagName),
        })
    }
}

impl<'xml> DeserializeContent<'xml> for LoggingEnabled {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut target_bucket: Option<TargetBucket> = None;
        let mut target_grants: Option<TargetGrants> = None;
        let mut target_prefix: Option<TargetPrefix> = None;
        d.for_each_element(|d, x| match x {
            b"TargetBucket" => {
                if target_bucket.is_some() {
                    return Err(DeError::DuplicateField);
                }
                target_bucket = Some(d.content()?);
                Ok(())
            }
            b"TargetGrants" => {
                if target_grants.is_some() {
                    return Err(DeError::DuplicateField);
                }
                target_grants = Some(d.list_content("member")?);
                Ok(())
            }
            b"TargetPrefix" => {
                if target_prefix.is_some() {
                    return Err(DeError::DuplicateField);
                }
                target_prefix = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            target_bucket: target_bucket.ok_or(DeError::MissingField)?,
            target_grants,
            target_prefix: target_prefix.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for MFADelete {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for MetadataEntry {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut name: Option<MetadataKey> = None;
        let mut value: Option<MetadataValue> = None;
        d.for_each_element(|d, x| match x {
            b"Name" => {
                if name.is_some() {
                    return Err(DeError::DuplicateField);
                }
                name = Some(d.content()?);
                Ok(())
            }
            b"Value" => {
                if value.is_some() {
                    return Err(DeError::DuplicateField);
                }
                value = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { name, value })
    }
}

impl<'xml> DeserializeContent<'xml> for Metrics {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut event_threshold: Option<ReplicationTimeValue> = None;
        let mut status: Option<MetricsStatus> = None;
        d.for_each_element(|d, x| match x {
            b"EventThreshold" => {
                if event_threshold.is_some() {
                    return Err(DeError::DuplicateField);
                }
                event_threshold = Some(d.content()?);
                Ok(())
            }
            b"Status" => {
                if status.is_some() {
                    return Err(DeError::DuplicateField);
                }
                status = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            event_threshold,
            status: status.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for MetricsAndOperator {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut access_point_arn: Option<AccessPointArn> = None;
        let mut prefix: Option<Prefix> = None;
        let mut tags: Option<TagSet> = None;
        d.for_each_element(|d, x| match x {
            b"AccessPointArn" => {
                if access_point_arn.is_some() {
                    return Err(DeError::DuplicateField);
                }
                access_point_arn = Some(d.content()?);
                Ok(())
            }
            b"Prefix" => {
                if prefix.is_some() {
                    return Err(DeError::DuplicateField);
                }
                prefix = Some(d.content()?);
                Ok(())
            }
            b"Tag" => {
                let ans: Tag = d.content()?;
                tags.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            access_point_arn,
            prefix,
            tags,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for MetricsConfiguration {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut filter: Option<MetricsFilter> = None;
        let mut id: Option<MetricsId> = None;
        d.for_each_element(|d, x| match x {
            b"Filter" => {
                if filter.is_some() {
                    return Err(DeError::DuplicateField);
                }
                filter = Some(d.content()?);
                Ok(())
            }
            b"Id" => {
                if id.is_some() {
                    return Err(DeError::DuplicateField);
                }
                id = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            filter,
            id: id.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for MetricsFilter {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.element(|d, x| match x {
            b"AccessPointArn" => Ok(Self::AccessPointArn(d.content()?)),
            b"And" => Ok(Self::And(d.content()?)),
            b"Prefix" => Ok(Self::Prefix(d.content()?)),
            b"Tag" => Ok(Self::Tag(d.content()?)),
            _ => Err(DeError::UnexpectedTagName),
        })
    }
}

impl<'xml> DeserializeContent<'xml> for MetricsStatus {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for NoncurrentVersionExpiration {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut newer_noncurrent_versions: Option<VersionCount> = None;
        let mut noncurrent_days: Option<Days> = None;
        d.for_each_element(|d, x| match x {
            b"NewerNoncurrentVersions" => {
                if newer_noncurrent_versions.is_some() {
                    return Err(DeError::DuplicateField);
                }
                newer_noncurrent_versions = Some(d.content()?);
                Ok(())
            }
            b"NoncurrentDays" => {
                if noncurrent_days.is_some() {
                    return Err(DeError::DuplicateField);
                }
                noncurrent_days = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            newer_noncurrent_versions: newer_noncurrent_versions.unwrap_or(0),
            noncurrent_days: noncurrent_days.unwrap_or(0),
        })
    }
}

impl<'xml> DeserializeContent<'xml> for NoncurrentVersionTransition {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut newer_noncurrent_versions: Option<VersionCount> = None;
        let mut noncurrent_days: Option<Days> = None;
        let mut storage_class: Option<TransitionStorageClass> = None;
        d.for_each_element(|d, x| match x {
            b"NewerNoncurrentVersions" => {
                if newer_noncurrent_versions.is_some() {
                    return Err(DeError::DuplicateField);
                }
                newer_noncurrent_versions = Some(d.content()?);
                Ok(())
            }
            b"NoncurrentDays" => {
                if noncurrent_days.is_some() {
                    return Err(DeError::DuplicateField);
                }
                noncurrent_days = Some(d.content()?);
                Ok(())
            }
            b"StorageClass" => {
                if storage_class.is_some() {
                    return Err(DeError::DuplicateField);
                }
                storage_class = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            newer_noncurrent_versions: newer_noncurrent_versions.unwrap_or(0),
            noncurrent_days: noncurrent_days.unwrap_or(0),
            storage_class,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for NotificationConfiguration {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut event_bridge_configuration: Option<EventBridgeConfiguration> = None;
        let mut lambda_function_configurations: Option<LambdaFunctionConfigurationList> = None;
        let mut queue_configurations: Option<QueueConfigurationList> = None;
        let mut topic_configurations: Option<TopicConfigurationList> = None;
        d.for_each_element(|d, x| match x {
            b"EventBridgeConfiguration" => {
                if event_bridge_configuration.is_some() {
                    return Err(DeError::DuplicateField);
                }
                event_bridge_configuration = Some(d.content()?);
                Ok(())
            }
            b"CloudFunctionConfiguration" => {
                let ans: LambdaFunctionConfiguration = d.content()?;
                lambda_function_configurations.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            b"QueueConfiguration" => {
                let ans: QueueConfiguration = d.content()?;
                queue_configurations.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            b"TopicConfiguration" => {
                let ans: TopicConfiguration = d.content()?;
                topic_configurations.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            event_bridge_configuration,
            lambda_function_configurations,
            queue_configurations,
            topic_configurations,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for NotificationConfigurationFilter {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut key: Option<S3KeyFilter> = None;
        d.for_each_element(|d, x| match x {
            b"S3Key" => {
                if key.is_some() {
                    return Err(DeError::DuplicateField);
                }
                key = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { key })
    }
}

impl<'xml> DeserializeContent<'xml> for ObjectCannedACL {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for ObjectIdentifier {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut key: Option<ObjectKey> = None;
        let mut version_id: Option<ObjectVersionId> = None;
        d.for_each_element(|d, x| match x {
            b"Key" => {
                if key.is_some() {
                    return Err(DeError::DuplicateField);
                }
                key = Some(d.content()?);
                Ok(())
            }
            b"VersionId" => {
                if version_id.is_some() {
                    return Err(DeError::DuplicateField);
                }
                version_id = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            key: key.ok_or(DeError::MissingField)?,
            version_id,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for ObjectLockConfiguration {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut object_lock_enabled: Option<ObjectLockEnabled> = None;
        let mut rule: Option<ObjectLockRule> = None;
        d.for_each_element(|d, x| match x {
            b"ObjectLockEnabled" => {
                if object_lock_enabled.is_some() {
                    return Err(DeError::DuplicateField);
                }
                object_lock_enabled = Some(d.content()?);
                Ok(())
            }
            b"Rule" => {
                if rule.is_some() {
                    return Err(DeError::DuplicateField);
                }
                rule = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            object_lock_enabled,
            rule,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for ObjectLockEnabled {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for ObjectLockLegalHold {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut status: Option<ObjectLockLegalHoldStatus> = None;
        d.for_each_element(|d, x| match x {
            b"Status" => {
                if status.is_some() {
                    return Err(DeError::DuplicateField);
                }
                status = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { status })
    }
}

impl<'xml> DeserializeContent<'xml> for ObjectLockLegalHoldStatus {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for ObjectLockRetention {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut mode: Option<ObjectLockRetentionMode> = None;
        let mut retain_until_date: Option<Date> = None;
        d.for_each_element(|d, x| match x {
            b"Mode" => {
                if mode.is_some() {
                    return Err(DeError::DuplicateField);
                }
                mode = Some(d.content()?);
                Ok(())
            }
            b"RetainUntilDate" => {
                if retain_until_date.is_some() {
                    return Err(DeError::DuplicateField);
                }
                retain_until_date = Some(d.timestamp(TimestampFormat::DateTime)?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { mode, retain_until_date })
    }
}

impl<'xml> DeserializeContent<'xml> for ObjectLockRetentionMode {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for ObjectLockRule {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut default_retention: Option<DefaultRetention> = None;
        d.for_each_element(|d, x| match x {
            b"DefaultRetention" => {
                if default_retention.is_some() {
                    return Err(DeError::DuplicateField);
                }
                default_retention = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { default_retention })
    }
}

impl<'xml> DeserializeContent<'xml> for ObjectOwnership {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for OutputLocation {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut s3: Option<S3Location> = None;
        d.for_each_element(|d, x| match x {
            b"S3" => {
                if s3.is_some() {
                    return Err(DeError::DuplicateField);
                }
                s3 = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { s3 })
    }
}

impl<'xml> DeserializeContent<'xml> for OutputSerialization {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut csv: Option<CSVOutput> = None;
        let mut json: Option<JSONOutput> = None;
        d.for_each_element(|d, x| match x {
            b"CSV" => {
                if csv.is_some() {
                    return Err(DeError::DuplicateField);
                }
                csv = Some(d.content()?);
                Ok(())
            }
            b"JSON" => {
                if json.is_some() {
                    return Err(DeError::DuplicateField);
                }
                json = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { csv, json })
    }
}

impl<'xml> DeserializeContent<'xml> for Owner {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut display_name: Option<DisplayName> = None;
        let mut id: Option<ID> = None;
        d.for_each_element(|d, x| match x {
            b"DisplayName" => {
                if display_name.is_some() {
                    return Err(DeError::DuplicateField);
                }
                display_name = Some(d.content()?);
                Ok(())
            }
            b"ID" => {
                if id.is_some() {
                    return Err(DeError::DuplicateField);
                }
                id = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { display_name, id })
    }
}

impl<'xml> DeserializeContent<'xml> for OwnerOverride {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for OwnershipControls {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut rules: Option<OwnershipControlsRules> = None;
        d.for_each_element(|d, x| match x {
            b"Rule" => {
                let ans: OwnershipControlsRule = d.content()?;
                rules.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            rules: rules.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for OwnershipControlsRule {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut object_ownership: Option<ObjectOwnership> = None;
        d.for_each_element(|d, x| match x {
            b"ObjectOwnership" => {
                if object_ownership.is_some() {
                    return Err(DeError::DuplicateField);
                }
                object_ownership = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            object_ownership: object_ownership.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for ParquetInput {
    fn deserialize_content(_: &mut Deserializer<'xml>) -> DeResult<Self> {
        Ok(Self {})
    }
}

impl<'xml> DeserializeContent<'xml> for Payer {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for Permission {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for Progress {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut bytes_processed: Option<BytesProcessed> = None;
        let mut bytes_returned: Option<BytesReturned> = None;
        let mut bytes_scanned: Option<BytesScanned> = None;
        d.for_each_element(|d, x| match x {
            b"BytesProcessed" => {
                if bytes_processed.is_some() {
                    return Err(DeError::DuplicateField);
                }
                bytes_processed = Some(d.content()?);
                Ok(())
            }
            b"BytesReturned" => {
                if bytes_returned.is_some() {
                    return Err(DeError::DuplicateField);
                }
                bytes_returned = Some(d.content()?);
                Ok(())
            }
            b"BytesScanned" => {
                if bytes_scanned.is_some() {
                    return Err(DeError::DuplicateField);
                }
                bytes_scanned = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            bytes_processed: bytes_processed.unwrap_or(0),
            bytes_returned: bytes_returned.unwrap_or(0),
            bytes_scanned: bytes_scanned.unwrap_or(0),
        })
    }
}

impl<'xml> DeserializeContent<'xml> for Protocol {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for PublicAccessBlockConfiguration {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut block_public_acls: Option<Setting> = None;
        let mut block_public_policy: Option<Setting> = None;
        let mut ignore_public_acls: Option<Setting> = None;
        let mut restrict_public_buckets: Option<Setting> = None;
        d.for_each_element(|d, x| match x {
            b"BlockPublicAcls" => {
                if block_public_acls.is_some() {
                    return Err(DeError::DuplicateField);
                }
                block_public_acls = Some(d.content()?);
                Ok(())
            }
            b"BlockPublicPolicy" => {
                if block_public_policy.is_some() {
                    return Err(DeError::DuplicateField);
                }
                block_public_policy = Some(d.content()?);
                Ok(())
            }
            b"IgnorePublicAcls" => {
                if ignore_public_acls.is_some() {
                    return Err(DeError::DuplicateField);
                }
                ignore_public_acls = Some(d.content()?);
                Ok(())
            }
            b"RestrictPublicBuckets" => {
                if restrict_public_buckets.is_some() {
                    return Err(DeError::DuplicateField);
                }
                restrict_public_buckets = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            block_public_acls: block_public_acls.unwrap_or(false),
            block_public_policy: block_public_policy.unwrap_or(false),
            ignore_public_acls: ignore_public_acls.unwrap_or(false),
            restrict_public_buckets: restrict_public_buckets.unwrap_or(false),
        })
    }
}

impl<'xml> DeserializeContent<'xml> for QueueConfiguration {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut events: Option<EventList> = None;
        let mut filter: Option<NotificationConfigurationFilter> = None;
        let mut id: Option<NotificationId> = None;
        let mut queue_arn: Option<QueueArn> = None;
        d.for_each_element(|d, x| match x {
            b"Event" => {
                let ans: Event = d.content()?;
                events.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            b"Filter" => {
                if filter.is_some() {
                    return Err(DeError::DuplicateField);
                }
                filter = Some(d.content()?);
                Ok(())
            }
            b"Id" => {
                if id.is_some() {
                    return Err(DeError::DuplicateField);
                }
                id = Some(d.content()?);
                Ok(())
            }
            b"Queue" => {
                if queue_arn.is_some() {
                    return Err(DeError::DuplicateField);
                }
                queue_arn = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            events: events.ok_or(DeError::MissingField)?,
            filter,
            id,
            queue_arn: queue_arn.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for QuoteFields {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for Redirect {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut host_name: Option<HostName> = None;
        let mut http_redirect_code: Option<HttpRedirectCode> = None;
        let mut protocol: Option<Protocol> = None;
        let mut replace_key_prefix_with: Option<ReplaceKeyPrefixWith> = None;
        let mut replace_key_with: Option<ReplaceKeyWith> = None;
        d.for_each_element(|d, x| match x {
            b"HostName" => {
                if host_name.is_some() {
                    return Err(DeError::DuplicateField);
                }
                host_name = Some(d.content()?);
                Ok(())
            }
            b"HttpRedirectCode" => {
                if http_redirect_code.is_some() {
                    return Err(DeError::DuplicateField);
                }
                http_redirect_code = Some(d.content()?);
                Ok(())
            }
            b"Protocol" => {
                if protocol.is_some() {
                    return Err(DeError::DuplicateField);
                }
                protocol = Some(d.content()?);
                Ok(())
            }
            b"ReplaceKeyPrefixWith" => {
                if replace_key_prefix_with.is_some() {
                    return Err(DeError::DuplicateField);
                }
                replace_key_prefix_with = Some(d.content()?);
                Ok(())
            }
            b"ReplaceKeyWith" => {
                if replace_key_with.is_some() {
                    return Err(DeError::DuplicateField);
                }
                replace_key_with = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            host_name,
            http_redirect_code,
            protocol,
            replace_key_prefix_with,
            replace_key_with,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for RedirectAllRequestsTo {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut host_name: Option<HostName> = None;
        let mut protocol: Option<Protocol> = None;
        d.for_each_element(|d, x| match x {
            b"HostName" => {
                if host_name.is_some() {
                    return Err(DeError::DuplicateField);
                }
                host_name = Some(d.content()?);
                Ok(())
            }
            b"Protocol" => {
                if protocol.is_some() {
                    return Err(DeError::DuplicateField);
                }
                protocol = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            host_name: host_name.ok_or(DeError::MissingField)?,
            protocol,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for ReplicaModifications {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut status: Option<ReplicaModificationsStatus> = None;
        d.for_each_element(|d, x| match x {
            b"Status" => {
                if status.is_some() {
                    return Err(DeError::DuplicateField);
                }
                status = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            status: status.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for ReplicaModificationsStatus {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for ReplicationConfiguration {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut role: Option<Role> = None;
        let mut rules: Option<ReplicationRules> = None;
        d.for_each_element(|d, x| match x {
            b"Role" => {
                if role.is_some() {
                    return Err(DeError::DuplicateField);
                }
                role = Some(d.content()?);
                Ok(())
            }
            b"Rule" => {
                let ans: ReplicationRule = d.content()?;
                rules.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            role: role.ok_or(DeError::MissingField)?,
            rules: rules.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for ReplicationRule {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut delete_marker_replication: Option<DeleteMarkerReplication> = None;
        let mut destination: Option<Destination> = None;
        let mut existing_object_replication: Option<ExistingObjectReplication> = None;
        let mut filter: Option<ReplicationRuleFilter> = None;
        let mut id: Option<ID> = None;
        let mut prefix: Option<Prefix> = None;
        let mut priority: Option<Priority> = None;
        let mut source_selection_criteria: Option<SourceSelectionCriteria> = None;
        let mut status: Option<ReplicationRuleStatus> = None;
        d.for_each_element(|d, x| match x {
            b"DeleteMarkerReplication" => {
                if delete_marker_replication.is_some() {
                    return Err(DeError::DuplicateField);
                }
                delete_marker_replication = Some(d.content()?);
                Ok(())
            }
            b"Destination" => {
                if destination.is_some() {
                    return Err(DeError::DuplicateField);
                }
                destination = Some(d.content()?);
                Ok(())
            }
            b"ExistingObjectReplication" => {
                if existing_object_replication.is_some() {
                    return Err(DeError::DuplicateField);
                }
                existing_object_replication = Some(d.content()?);
                Ok(())
            }
            b"Filter" => {
                if filter.is_some() {
                    return Err(DeError::DuplicateField);
                }
                filter = Some(d.content()?);
                Ok(())
            }
            b"ID" => {
                if id.is_some() {
                    return Err(DeError::DuplicateField);
                }
                id = Some(d.content()?);
                Ok(())
            }
            b"Prefix" => {
                if prefix.is_some() {
                    return Err(DeError::DuplicateField);
                }
                prefix = Some(d.content()?);
                Ok(())
            }
            b"Priority" => {
                if priority.is_some() {
                    return Err(DeError::DuplicateField);
                }
                priority = Some(d.content()?);
                Ok(())
            }
            b"SourceSelectionCriteria" => {
                if source_selection_criteria.is_some() {
                    return Err(DeError::DuplicateField);
                }
                source_selection_criteria = Some(d.content()?);
                Ok(())
            }
            b"Status" => {
                if status.is_some() {
                    return Err(DeError::DuplicateField);
                }
                status = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            delete_marker_replication,
            destination: destination.ok_or(DeError::MissingField)?,
            existing_object_replication,
            filter,
            id,
            prefix,
            priority: priority.unwrap_or(0),
            source_selection_criteria,
            status: status.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for ReplicationRuleAndOperator {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut prefix: Option<Prefix> = None;
        let mut tags: Option<TagSet> = None;
        d.for_each_element(|d, x| match x {
            b"Prefix" => {
                if prefix.is_some() {
                    return Err(DeError::DuplicateField);
                }
                prefix = Some(d.content()?);
                Ok(())
            }
            b"Tag" => {
                let ans: Tag = d.content()?;
                tags.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { prefix, tags })
    }
}

impl<'xml> DeserializeContent<'xml> for ReplicationRuleFilter {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.element(|d, x| match x {
            b"And" => Ok(Self::And(d.content()?)),
            b"Prefix" => Ok(Self::Prefix(d.content()?)),
            b"Tag" => Ok(Self::Tag(d.content()?)),
            _ => Err(DeError::UnexpectedTagName),
        })
    }
}

impl<'xml> DeserializeContent<'xml> for ReplicationRuleStatus {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for ReplicationTime {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut status: Option<ReplicationTimeStatus> = None;
        let mut time: Option<ReplicationTimeValue> = None;
        d.for_each_element(|d, x| match x {
            b"Status" => {
                if status.is_some() {
                    return Err(DeError::DuplicateField);
                }
                status = Some(d.content()?);
                Ok(())
            }
            b"Time" => {
                if time.is_some() {
                    return Err(DeError::DuplicateField);
                }
                time = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            status: status.ok_or(DeError::MissingField)?,
            time: time.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for ReplicationTimeStatus {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for ReplicationTimeValue {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut minutes: Option<Minutes> = None;
        d.for_each_element(|d, x| match x {
            b"Minutes" => {
                if minutes.is_some() {
                    return Err(DeError::DuplicateField);
                }
                minutes = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            minutes: minutes.unwrap_or(0),
        })
    }
}

impl<'xml> DeserializeContent<'xml> for RequestPaymentConfiguration {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut payer: Option<Payer> = None;
        d.for_each_element(|d, x| match x {
            b"Payer" => {
                if payer.is_some() {
                    return Err(DeError::DuplicateField);
                }
                payer = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            payer: payer.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for RequestProgress {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut enabled: Option<EnableRequestProgress> = None;
        d.for_each_element(|d, x| match x {
            b"Enabled" => {
                if enabled.is_some() {
                    return Err(DeError::DuplicateField);
                }
                enabled = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            enabled: enabled.unwrap_or(false),
        })
    }
}

impl<'xml> DeserializeContent<'xml> for RestoreRequest {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut days: Option<Days> = None;
        let mut description: Option<Description> = None;
        let mut glacier_job_parameters: Option<GlacierJobParameters> = None;
        let mut output_location: Option<OutputLocation> = None;
        let mut select_parameters: Option<SelectParameters> = None;
        let mut tier: Option<Tier> = None;
        let mut type_: Option<RestoreRequestType> = None;
        d.for_each_element(|d, x| match x {
            b"Days" => {
                if days.is_some() {
                    return Err(DeError::DuplicateField);
                }
                days = Some(d.content()?);
                Ok(())
            }
            b"Description" => {
                if description.is_some() {
                    return Err(DeError::DuplicateField);
                }
                description = Some(d.content()?);
                Ok(())
            }
            b"GlacierJobParameters" => {
                if glacier_job_parameters.is_some() {
                    return Err(DeError::DuplicateField);
                }
                glacier_job_parameters = Some(d.content()?);
                Ok(())
            }
            b"OutputLocation" => {
                if output_location.is_some() {
                    return Err(DeError::DuplicateField);
                }
                output_location = Some(d.content()?);
                Ok(())
            }
            b"SelectParameters" => {
                if select_parameters.is_some() {
                    return Err(DeError::DuplicateField);
                }
                select_parameters = Some(d.content()?);
                Ok(())
            }
            b"Tier" => {
                if tier.is_some() {
                    return Err(DeError::DuplicateField);
                }
                tier = Some(d.content()?);
                Ok(())
            }
            b"Type" => {
                if type_.is_some() {
                    return Err(DeError::DuplicateField);
                }
                type_ = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            days: days.unwrap_or(0),
            description,
            glacier_job_parameters,
            output_location,
            select_parameters,
            tier,
            type_,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for RestoreRequestType {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for RoutingRule {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut condition: Option<Condition> = None;
        let mut redirect: Option<Redirect> = None;
        d.for_each_element(|d, x| match x {
            b"Condition" => {
                if condition.is_some() {
                    return Err(DeError::DuplicateField);
                }
                condition = Some(d.content()?);
                Ok(())
            }
            b"Redirect" => {
                if redirect.is_some() {
                    return Err(DeError::DuplicateField);
                }
                redirect = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            condition,
            redirect: redirect.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for S3KeyFilter {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut filter_rules: Option<FilterRuleList> = None;
        d.for_each_element(|d, x| match x {
            b"FilterRule" => {
                let ans: FilterRule = d.content()?;
                filter_rules.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { filter_rules })
    }
}

impl<'xml> DeserializeContent<'xml> for S3Location {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut access_control_list: Option<Grants> = None;
        let mut bucket_name: Option<BucketName> = None;
        let mut canned_acl: Option<ObjectCannedACL> = None;
        let mut encryption: Option<Encryption> = None;
        let mut prefix: Option<LocationPrefix> = None;
        let mut storage_class: Option<StorageClass> = None;
        let mut tagging: Option<Tagging> = None;
        let mut user_metadata: Option<UserMetadata> = None;
        d.for_each_element(|d, x| match x {
            b"AccessControlList" => {
                if access_control_list.is_some() {
                    return Err(DeError::DuplicateField);
                }
                access_control_list = Some(d.list_content("member")?);
                Ok(())
            }
            b"BucketName" => {
                if bucket_name.is_some() {
                    return Err(DeError::DuplicateField);
                }
                bucket_name = Some(d.content()?);
                Ok(())
            }
            b"CannedACL" => {
                if canned_acl.is_some() {
                    return Err(DeError::DuplicateField);
                }
                canned_acl = Some(d.content()?);
                Ok(())
            }
            b"Encryption" => {
                if encryption.is_some() {
                    return Err(DeError::DuplicateField);
                }
                encryption = Some(d.content()?);
                Ok(())
            }
            b"Prefix" => {
                if prefix.is_some() {
                    return Err(DeError::DuplicateField);
                }
                prefix = Some(d.content()?);
                Ok(())
            }
            b"StorageClass" => {
                if storage_class.is_some() {
                    return Err(DeError::DuplicateField);
                }
                storage_class = Some(d.content()?);
                Ok(())
            }
            b"Tagging" => {
                if tagging.is_some() {
                    return Err(DeError::DuplicateField);
                }
                tagging = Some(d.content()?);
                Ok(())
            }
            b"UserMetadata" => {
                if user_metadata.is_some() {
                    return Err(DeError::DuplicateField);
                }
                user_metadata = Some(d.list_content("member")?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            access_control_list,
            bucket_name: bucket_name.ok_or(DeError::MissingField)?,
            canned_acl,
            encryption,
            prefix: prefix.ok_or(DeError::MissingField)?,
            storage_class,
            tagging,
            user_metadata,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for SSEKMS {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut key_id: Option<SSEKMSKeyId> = None;
        d.for_each_element(|d, x| match x {
            b"KeyId" => {
                if key_id.is_some() {
                    return Err(DeError::DuplicateField);
                }
                key_id = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            key_id: key_id.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for SSES3 {
    fn deserialize_content(_: &mut Deserializer<'xml>) -> DeResult<Self> {
        Ok(Self {})
    }
}

impl<'xml> DeserializeContent<'xml> for ScanRange {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut end: Option<End> = None;
        let mut start: Option<Start> = None;
        d.for_each_element(|d, x| match x {
            b"End" => {
                if end.is_some() {
                    return Err(DeError::DuplicateField);
                }
                end = Some(d.content()?);
                Ok(())
            }
            b"Start" => {
                if start.is_some() {
                    return Err(DeError::DuplicateField);
                }
                start = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            end: end.unwrap_or(0),
            start: start.unwrap_or(0),
        })
    }
}

impl<'xml> DeserializeContent<'xml> for SelectObjectContentRequest {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut expression: Option<Expression> = None;
        let mut expression_type: Option<ExpressionType> = None;
        let mut input_serialization: Option<InputSerialization> = None;
        let mut output_serialization: Option<OutputSerialization> = None;
        let mut request_progress: Option<RequestProgress> = None;
        let mut scan_range: Option<ScanRange> = None;
        d.for_each_element(|d, x| match x {
            b"Expression" => {
                if expression.is_some() {
                    return Err(DeError::DuplicateField);
                }
                expression = Some(d.content()?);
                Ok(())
            }
            b"ExpressionType" => {
                if expression_type.is_some() {
                    return Err(DeError::DuplicateField);
                }
                expression_type = Some(d.content()?);
                Ok(())
            }
            b"InputSerialization" => {
                if input_serialization.is_some() {
                    return Err(DeError::DuplicateField);
                }
                input_serialization = Some(d.content()?);
                Ok(())
            }
            b"OutputSerialization" => {
                if output_serialization.is_some() {
                    return Err(DeError::DuplicateField);
                }
                output_serialization = Some(d.content()?);
                Ok(())
            }
            b"RequestProgress" => {
                if request_progress.is_some() {
                    return Err(DeError::DuplicateField);
                }
                request_progress = Some(d.content()?);
                Ok(())
            }
            b"ScanRange" => {
                if scan_range.is_some() {
                    return Err(DeError::DuplicateField);
                }
                scan_range = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            expression: expression.ok_or(DeError::MissingField)?,
            expression_type: expression_type.ok_or(DeError::MissingField)?,
            input_serialization: input_serialization.ok_or(DeError::MissingField)?,
            output_serialization: output_serialization.ok_or(DeError::MissingField)?,
            request_progress,
            scan_range,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for SelectParameters {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut expression: Option<Expression> = None;
        let mut expression_type: Option<ExpressionType> = None;
        let mut input_serialization: Option<InputSerialization> = None;
        let mut output_serialization: Option<OutputSerialization> = None;
        d.for_each_element(|d, x| match x {
            b"Expression" => {
                if expression.is_some() {
                    return Err(DeError::DuplicateField);
                }
                expression = Some(d.content()?);
                Ok(())
            }
            b"ExpressionType" => {
                if expression_type.is_some() {
                    return Err(DeError::DuplicateField);
                }
                expression_type = Some(d.content()?);
                Ok(())
            }
            b"InputSerialization" => {
                if input_serialization.is_some() {
                    return Err(DeError::DuplicateField);
                }
                input_serialization = Some(d.content()?);
                Ok(())
            }
            b"OutputSerialization" => {
                if output_serialization.is_some() {
                    return Err(DeError::DuplicateField);
                }
                output_serialization = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            expression: expression.ok_or(DeError::MissingField)?,
            expression_type: expression_type.ok_or(DeError::MissingField)?,
            input_serialization: input_serialization.ok_or(DeError::MissingField)?,
            output_serialization: output_serialization.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for ServerSideEncryption {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for ServerSideEncryptionByDefault {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut kms_master_key_id: Option<SSEKMSKeyId> = None;
        let mut sse_algorithm: Option<ServerSideEncryption> = None;
        d.for_each_element(|d, x| match x {
            b"KMSMasterKeyID" => {
                if kms_master_key_id.is_some() {
                    return Err(DeError::DuplicateField);
                }
                kms_master_key_id = Some(d.content()?);
                Ok(())
            }
            b"SSEAlgorithm" => {
                if sse_algorithm.is_some() {
                    return Err(DeError::DuplicateField);
                }
                sse_algorithm = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            kms_master_key_id,
            sse_algorithm: sse_algorithm.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for ServerSideEncryptionConfiguration {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut rules: Option<ServerSideEncryptionRules> = None;
        d.for_each_element(|d, x| match x {
            b"Rule" => {
                let ans: ServerSideEncryptionRule = d.content()?;
                rules.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            rules: rules.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for ServerSideEncryptionRule {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut apply_server_side_encryption_by_default: Option<ServerSideEncryptionByDefault> = None;
        let mut bucket_key_enabled: Option<BucketKeyEnabled> = None;
        d.for_each_element(|d, x| match x {
            b"ApplyServerSideEncryptionByDefault" => {
                if apply_server_side_encryption_by_default.is_some() {
                    return Err(DeError::DuplicateField);
                }
                apply_server_side_encryption_by_default = Some(d.content()?);
                Ok(())
            }
            b"BucketKeyEnabled" => {
                if bucket_key_enabled.is_some() {
                    return Err(DeError::DuplicateField);
                }
                bucket_key_enabled = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            apply_server_side_encryption_by_default,
            bucket_key_enabled: bucket_key_enabled.unwrap_or(false),
        })
    }
}

impl<'xml> DeserializeContent<'xml> for SourceSelectionCriteria {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut replica_modifications: Option<ReplicaModifications> = None;
        let mut sse_kms_encrypted_objects: Option<SseKmsEncryptedObjects> = None;
        d.for_each_element(|d, x| match x {
            b"ReplicaModifications" => {
                if replica_modifications.is_some() {
                    return Err(DeError::DuplicateField);
                }
                replica_modifications = Some(d.content()?);
                Ok(())
            }
            b"SseKmsEncryptedObjects" => {
                if sse_kms_encrypted_objects.is_some() {
                    return Err(DeError::DuplicateField);
                }
                sse_kms_encrypted_objects = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            replica_modifications,
            sse_kms_encrypted_objects,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for SseKmsEncryptedObjects {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut status: Option<SseKmsEncryptedObjectsStatus> = None;
        d.for_each_element(|d, x| match x {
            b"Status" => {
                if status.is_some() {
                    return Err(DeError::DuplicateField);
                }
                status = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            status: status.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for SseKmsEncryptedObjectsStatus {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for Stats {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut bytes_processed: Option<BytesProcessed> = None;
        let mut bytes_returned: Option<BytesReturned> = None;
        let mut bytes_scanned: Option<BytesScanned> = None;
        d.for_each_element(|d, x| match x {
            b"BytesProcessed" => {
                if bytes_processed.is_some() {
                    return Err(DeError::DuplicateField);
                }
                bytes_processed = Some(d.content()?);
                Ok(())
            }
            b"BytesReturned" => {
                if bytes_returned.is_some() {
                    return Err(DeError::DuplicateField);
                }
                bytes_returned = Some(d.content()?);
                Ok(())
            }
            b"BytesScanned" => {
                if bytes_scanned.is_some() {
                    return Err(DeError::DuplicateField);
                }
                bytes_scanned = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            bytes_processed: bytes_processed.unwrap_or(0),
            bytes_returned: bytes_returned.unwrap_or(0),
            bytes_scanned: bytes_scanned.unwrap_or(0),
        })
    }
}

impl<'xml> DeserializeContent<'xml> for StorageClass {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for StorageClassAnalysis {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut data_export: Option<StorageClassAnalysisDataExport> = None;
        d.for_each_element(|d, x| match x {
            b"DataExport" => {
                if data_export.is_some() {
                    return Err(DeError::DuplicateField);
                }
                data_export = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { data_export })
    }
}

impl<'xml> DeserializeContent<'xml> for StorageClassAnalysisDataExport {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut destination: Option<AnalyticsExportDestination> = None;
        let mut output_schema_version: Option<StorageClassAnalysisSchemaVersion> = None;
        d.for_each_element(|d, x| match x {
            b"Destination" => {
                if destination.is_some() {
                    return Err(DeError::DuplicateField);
                }
                destination = Some(d.content()?);
                Ok(())
            }
            b"OutputSchemaVersion" => {
                if output_schema_version.is_some() {
                    return Err(DeError::DuplicateField);
                }
                output_schema_version = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            destination: destination.ok_or(DeError::MissingField)?,
            output_schema_version: output_schema_version.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for StorageClassAnalysisSchemaVersion {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for Tag {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut key: Option<ObjectKey> = None;
        let mut value: Option<Value> = None;
        d.for_each_element(|d, x| match x {
            b"Key" => {
                if key.is_some() {
                    return Err(DeError::DuplicateField);
                }
                key = Some(d.content()?);
                Ok(())
            }
            b"Value" => {
                if value.is_some() {
                    return Err(DeError::DuplicateField);
                }
                value = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            key: key.ok_or(DeError::MissingField)?,
            value: value.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for Tagging {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut tag_set: Option<TagSet> = None;
        d.for_each_element(|d, x| match x {
            b"TagSet" => {
                if tag_set.is_some() {
                    return Err(DeError::DuplicateField);
                }
                tag_set = Some(d.list_content("member")?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            tag_set: tag_set.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for TargetGrant {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut grantee: Option<Grantee> = None;
        let mut permission: Option<BucketLogsPermission> = None;
        d.for_each_element(|d, x| match x {
            b"Grantee" => {
                if grantee.is_some() {
                    return Err(DeError::DuplicateField);
                }
                grantee = Some(d.content()?);
                Ok(())
            }
            b"Permission" => {
                if permission.is_some() {
                    return Err(DeError::DuplicateField);
                }
                permission = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { grantee, permission })
    }
}

impl<'xml> DeserializeContent<'xml> for Tier {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for Tiering {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut access_tier: Option<IntelligentTieringAccessTier> = None;
        let mut days: Option<IntelligentTieringDays> = None;
        d.for_each_element(|d, x| match x {
            b"AccessTier" => {
                if access_tier.is_some() {
                    return Err(DeError::DuplicateField);
                }
                access_tier = Some(d.content()?);
                Ok(())
            }
            b"Days" => {
                if days.is_some() {
                    return Err(DeError::DuplicateField);
                }
                days = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            access_tier: access_tier.ok_or(DeError::MissingField)?,
            days: days.unwrap_or(0),
        })
    }
}

impl<'xml> DeserializeContent<'xml> for TopicConfiguration {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut events: Option<EventList> = None;
        let mut filter: Option<NotificationConfigurationFilter> = None;
        let mut id: Option<NotificationId> = None;
        let mut topic_arn: Option<TopicArn> = None;
        d.for_each_element(|d, x| match x {
            b"Event" => {
                let ans: Event = d.content()?;
                events.get_or_insert_with(List::new).push(ans);
                Ok(())
            }
            b"Filter" => {
                if filter.is_some() {
                    return Err(DeError::DuplicateField);
                }
                filter = Some(d.content()?);
                Ok(())
            }
            b"Id" => {
                if id.is_some() {
                    return Err(DeError::DuplicateField);
                }
                id = Some(d.content()?);
                Ok(())
            }
            b"Topic" => {
                if topic_arn.is_some() {
                    return Err(DeError::DuplicateField);
                }
                topic_arn = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            events: events.ok_or(DeError::MissingField)?,
            filter,
            id,
            topic_arn: topic_arn.ok_or(DeError::MissingField)?,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for Transition {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut date: Option<Date> = None;
        let mut days: Option<Days> = None;
        let mut storage_class: Option<TransitionStorageClass> = None;
        d.for_each_element(|d, x| match x {
            b"Date" => {
                if date.is_some() {
                    return Err(DeError::DuplicateField);
                }
                date = Some(d.timestamp(TimestampFormat::DateTime)?);
                Ok(())
            }
            b"Days" => {
                if days.is_some() {
                    return Err(DeError::DuplicateField);
                }
                days = Some(d.content()?);
                Ok(())
            }
            b"StorageClass" => {
                if storage_class.is_some() {
                    return Err(DeError::DuplicateField);
                }
                storage_class = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            date,
            days: days.unwrap_or(0),
            storage_class,
        })
    }
}

impl<'xml> DeserializeContent<'xml> for TransitionStorageClass {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for Type {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        String::deserialize_content(d).map(Self::from)
    }
}

impl<'xml> DeserializeContent<'xml> for VersioningConfiguration {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut mfa_delete: Option<MFADelete> = None;
        let mut status: Option<BucketVersioningStatus> = None;
        d.for_each_element(|d, x| match x {
            b"MfaDelete" => {
                if mfa_delete.is_some() {
                    return Err(DeError::DuplicateField);
                }
                mfa_delete = Some(d.content()?);
                Ok(())
            }
            b"Status" => {
                if status.is_some() {
                    return Err(DeError::DuplicateField);
                }
                status = Some(d.content()?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self { mfa_delete, status })
    }
}

impl<'xml> DeserializeContent<'xml> for WebsiteConfiguration {
    fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        let mut error_document: Option<ErrorDocument> = None;
        let mut index_document: Option<IndexDocument> = None;
        let mut redirect_all_requests_to: Option<RedirectAllRequestsTo> = None;
        let mut routing_rules: Option<RoutingRules> = None;
        d.for_each_element(|d, x| match x {
            b"ErrorDocument" => {
                if error_document.is_some() {
                    return Err(DeError::DuplicateField);
                }
                error_document = Some(d.content()?);
                Ok(())
            }
            b"IndexDocument" => {
                if index_document.is_some() {
                    return Err(DeError::DuplicateField);
                }
                index_document = Some(d.content()?);
                Ok(())
            }
            b"RedirectAllRequestsTo" => {
                if redirect_all_requests_to.is_some() {
                    return Err(DeError::DuplicateField);
                }
                redirect_all_requests_to = Some(d.content()?);
                Ok(())
            }
            b"RoutingRules" => {
                if routing_rules.is_some() {
                    return Err(DeError::DuplicateField);
                }
                routing_rules = Some(d.list_content("member")?);
                Ok(())
            }
            _ => Err(DeError::UnexpectedTagName),
        })?;
        Ok(Self {
            error_document,
            index_document,
            redirect_all_requests_to,
            routing_rules,
        })
    }
}

impl<'xml> Deserialize<'xml> for AccelerateConfiguration {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("AccelerateConfiguration", |d| d.content())
    }
}

impl<'xml> Deserialize<'xml> for AccessControlPolicy {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("AccessControlPolicy", |d| d.content())
    }
}

impl<'xml> Deserialize<'xml> for AnalyticsConfiguration {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("AnalyticsConfiguration", |d| d.content())
    }
}

impl<'xml> Deserialize<'xml> for BucketLifecycleConfiguration {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("LifecycleConfiguration", |d| d.content())
    }
}

impl<'xml> Deserialize<'xml> for BucketLoggingStatus {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("BucketLoggingStatus", |d| d.content())
    }
}

impl<'xml> Deserialize<'xml> for CORSConfiguration {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("CORSConfiguration", |d| d.content())
    }
}

impl<'xml> Deserialize<'xml> for CompletedMultipartUpload {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("CompleteMultipartUpload", |d| d.content())
    }
}

impl<'xml> Deserialize<'xml> for CreateBucketConfiguration {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("CreateBucketConfiguration", |d| d.content())
    }
}

impl<'xml> Deserialize<'xml> for Delete {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("Delete", |d| d.content())
    }
}

impl<'xml> Deserialize<'xml> for IntelligentTieringConfiguration {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("IntelligentTieringConfiguration", |d| d.content())
    }
}

impl<'xml> Deserialize<'xml> for InventoryConfiguration {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("InventoryConfiguration", |d| d.content())
    }
}

impl<'xml> Deserialize<'xml> for MetricsConfiguration {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("MetricsConfiguration", |d| d.content())
    }
}

impl<'xml> Deserialize<'xml> for NotificationConfiguration {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("NotificationConfiguration", |d| d.content())
    }
}

impl<'xml> Deserialize<'xml> for ObjectLockConfiguration {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("ObjectLockConfiguration", |d| d.content())
    }
}

impl<'xml> Deserialize<'xml> for ObjectLockLegalHold {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("LegalHold", |d| d.content())
    }
}

impl<'xml> Deserialize<'xml> for ObjectLockRetention {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("Retention", |d| d.content())
    }
}

impl<'xml> Deserialize<'xml> for OwnershipControls {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("OwnershipControls", |d| d.content())
    }
}

impl<'xml> Deserialize<'xml> for Progress {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("Progress", |d| d.content())
    }
}

impl<'xml> Deserialize<'xml> for PublicAccessBlockConfiguration {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("PublicAccessBlockConfiguration", |d| d.content())
    }
}

impl<'xml> Deserialize<'xml> for ReplicationConfiguration {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("ReplicationConfiguration", |d| d.content())
    }
}

impl<'xml> Deserialize<'xml> for RequestPaymentConfiguration {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("RequestPaymentConfiguration", |d| d.content())
    }
}

impl<'xml> Deserialize<'xml> for RestoreRequest {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("RestoreRequest", |d| d.content())
    }
}

impl<'xml> Deserialize<'xml> for SelectObjectContentRequest {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("SelectObjectContentRequest", |d| d.content())
    }
}

impl<'xml> Deserialize<'xml> for ServerSideEncryptionConfiguration {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("ServerSideEncryptionConfiguration", |d| d.content())
    }
}

impl<'xml> Deserialize<'xml> for Stats {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("Stats", |d| d.content())
    }
}

impl<'xml> Deserialize<'xml> for Tagging {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("Tagging", |d| d.content())
    }
}

impl<'xml> Deserialize<'xml> for VersioningConfiguration {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("VersioningConfiguration", |d| d.content())
    }
}

impl<'xml> Deserialize<'xml> for WebsiteConfiguration {
    fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
        d.named_element("WebsiteConfiguration", |d| d.content())
    }
}
