import os

path = "/proc/"

for f in os.listdir(path):
    print(os.path.join(path, f))
