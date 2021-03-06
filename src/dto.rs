//! S3 data transfer objects

pub use rusoto_core::ByteStream;
pub use rusoto_s3::{
    Bucket, CompleteMultipartUploadError, CompleteMultipartUploadOutput,
    CompleteMultipartUploadRequest, CompletedMultipartUpload, CompletedPart, CopyObjectError,
    CopyObjectOutput, CopyObjectRequest, CopyObjectResult, CreateBucketConfiguration,
    CreateBucketError, CreateBucketOutput, CreateBucketRequest, CreateMultipartUploadError,
    CreateMultipartUploadOutput, CreateMultipartUploadRequest, Delete, DeleteBucketError,
    DeleteBucketRequest, DeleteObjectError, DeleteObjectOutput, DeleteObjectRequest,
    DeleteObjectsError, DeleteObjectsOutput, DeleteObjectsRequest, DeletedObject,
    GetBucketLocationError, GetBucketLocationOutput, GetBucketLocationRequest, GetObjectError,
    GetObjectOutput, GetObjectRequest, HeadBucketError, HeadBucketRequest, HeadObjectError,
    HeadObjectOutput, HeadObjectRequest, ListBucketsError, ListBucketsOutput, ListObjectsError,
    ListObjectsOutput, ListObjectsRequest, ListObjectsV2Error, ListObjectsV2Output,
    ListObjectsV2Request, Object, ObjectIdentifier, PutObjectError, PutObjectOutput,
    PutObjectRequest, UploadPartError, UploadPartOutput, UploadPartRequest,
};

/// `DeleteBucketOutput`
#[derive(Debug, Clone, Copy)]
pub struct DeleteBucketOutput;

/// `HeadBucketOutput`
#[derive(Debug, Clone, Copy)]
pub struct HeadBucketOutput;

/// `HeadBucketOutput`
#[derive(Debug, Clone, Copy)]
pub struct ListBucketsRequest;
