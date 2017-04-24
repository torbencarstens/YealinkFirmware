import boto3
import sys

try:
  file = sys.argv[1]
except IndexError:
  print("Please specify a file to upload as cmd argument")
  sys.exit(2)

s3 = boto3.resource('s3')
bucket_name = "yealink-firmware"
bucket = s3.Bucket(bucket_name)

try:
  short_name = file.split("-")[0]
except IndexError:
  print("Using filename({}) as key.".format(file))
  short_name = file.split(".rom", maxsplit=1)[0]

bucket.upload_file(file, '{}.rom'.format(short_name))
