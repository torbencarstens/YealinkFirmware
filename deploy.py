import sys

import boto3

try:
    file = sys.argv[1]
except IndexError:
    print("Please specify a file to upload as cmd argument")
    sys.exit(2)

try:
    bucket_name = sys.argv[2]
except IndexError:
    print("Using 'yealink-firmware' as bucket-name")
    bucket_name = "yealink-firmware"

region_name = "eu-central-1"
# config = botocore.config.Config(region_name=region_name, signature_version='v4')
# see http://boto3.readthedocs.io/en/latest/guide/configuration.html for setting the credentials
# s3 = boto3.resource('s3', region_name=region_name, config=config)
s3 = boto3.resource('s3', region_name=region_name)
bucket: s3.Bucket = s3.Bucket(bucket_name)

short_name = file.split("-")[0]
print("Using {} as key".format(short_name))

bucket.upload_file(file, '{}.rom'.format(short_name))
print("Successfully uploaded file {}".format(short_name))
print("URL: {}".format(f"https://s3.{region_name}.amazonaws.com/{bucket_name}/{short_name}"))
