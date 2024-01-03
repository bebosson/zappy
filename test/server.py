import subprocess

subprocess.run(["cargo", "build", "--bin", "server"])

print("- - - - - - - - - - - - - - - - - -\n\n\n")

error_test = [
"target/debug/server -x 2 -y 2 -p 1312 -t 1 -n lala -c",
"target/debug/server -x 2 -y 2 -p 1312 -t 1 -n -c 1",
"target/debug/server -x 2 -y 2 -p 1312 -t 1 -n lala -n -c 1",
"target/debug/server -x 2 -y 2 -p 1312 -t 1 -n lala -c 1 -n",
"target/debug/server -x 2 -y 2 -p 1312 -t 1 -n lala -c -n toto",
"target/debug/server -x 2 -y 2 -p 1312 -t 1 -n -n lala -c 1",
"target/debug/server -x 2 -y 2 -p 1312 -t 1 -n toto -n lala -c 1",
"target/debug/server -x 2 -y 2 -p 1312 -t 1 -n "" -c 1",
"target/debug/server -x 2 -y 2 -p 1312 -t 1 -n toto toto -c 1",
"target/debug/server -x 2 -y 2 -p 1312 -t 1 -n toto lala toto lala titi -c 1",
"target/debug/server -x 2 -y 2 -p 1312 -c 1 -t 1 -n toto lala titi -c 1",
]



for elem in error_test : 
    print(f"trying to test :    {elem}")
    result = subprocess.run(elem.split())
    if result.returncode == 0:
        print(f"--------> error for test {result.args}")
    print("- - - - - - - - - - - - - - - - - -")
    print("")

