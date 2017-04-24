import boto3
import sys

try:
  file = sys.argv[1]
except IndexError:
  print("Please specify a file to upload as cmd argument")
  sys.exit(2)

s3 = boto3.client('s3')
bucket_name = "yealink-firmware"

buckets = [bucket['Name'] for bucket in s3.list_buckets() if
           bucket_name['Name'] == bucket_name]

if buckets:
  bucket = buckets[0]
  try:
    short_name = file.split("-")[0]
  except IndexError:
    print("Using filename({}) as key.".format(file))
    short_name = file

  bucket.upload_file(file, '{}.rom'.format(short_name))
else:
  print("Couldn't find bucket 'yealink-firmware', aborting.")
  sys.exit(3)
