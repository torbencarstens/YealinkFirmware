import boto3
import botocore
import sys

try:
  file = sys.argv[1]
except IndexError:
  print("Please specify a file to upload as cmd argument")
  sys.exit(2)

try:
  bucket_name = sys.argv[2]
except IndexError:
  print("Using 'yealink-firmware' as bucket-name")
  bucket_name = yealink-firmware

region_name = "eu-central-1"
config = botocore.config.Config(region_name=region_name, signature_version='v4')
// see http://boto3.readthedocs.io/en/latest/guide/configuration.html for setting the credentials
s3 = boto3.resource('s3', region_name=region_name, config=config)
bucket = s3.Bucket(bucket_name)

try:
  short_name = file.split("-")[0]
except IndexError:
  print("Using filename({}) as key.".format(file))
  short_name = file.split(".rom", maxsplit=1)[0]

bucket.upload_file(file, '{}.rom'.format(short_name))
