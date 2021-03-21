import os
from pipeit import *
import shutil
os.system("cargo build --release")

for files in os.walk(os.path.abspath('./target/release')):
    files = files[2] | Filter(lambda x:os.path.splitext(x)[1] == '.dll') | list
    break

for file in files:
    target_name = os.path.join(os.path.abspath('./target/release') , os.path.splitext(file)[0]+'.pyd')
    orn_name = os.path.join(os.path.abspath('./target/release') , file)
    move_des = os.path.join(os.path.abspath('./') , os.path.splitext(file)[0]+'.pyd')
    if os.path.exists(target_name):
        os.remove(target_name)
    os.rename(orn_name , target_name)
    if os.path.exists(move_des):
        os.remove(move_des)
    shutil.copy(target_name , move_des)

os.system("python run.py")