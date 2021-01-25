#!/usr/bin/python3
import os, sys

"""
2 possible states:
>there is a makefile and it will compile it
>there is no makefile or its ignored and the argument is a filename


-h is help
-r delete after runnning 
-i ignore the makefile 

"""
class loop():
    def __init__(self, target):
        self.target = target;
        
    def exe_ord(self, ordr):
        os.system(ordr);

    def comp_this(self):
        pass 
        
    def run_this(self):
        ordr = "./"+str(self.target)
        
    def run_loop(self):
        self.comp_this()
        self.run_this()
        

        
class basic_loop(loop):
    def __init__(self, target):
        loop.__init__(self, target)

    def __repr__(self):
        string = "basic loop "+ str(target)
        return string
    
    def delet_this(self):
        ordr = "rm "+ self.target
        self.exe_ord(ordr);

    def comp_this(self):
        ordr = "make "+ self.target
        self.exe_ord(ordr)


class makefile_loop(loop):
    def __init__(self, target):
        loop.__init__(self, target)
        
    def __repr__(self):
        string = "makefile loop "+ str(target)
        return string
        
    def delet_this(self, x):
        ordr = "make clean"
        self.exe_ord(ordr);

    def comp_this(self):
        ordr = "make"
        self.exe_ord(ordr)

    def run_loop(self):
        self.run_this()

        
    
class pycma():
    def __init__(self):
        self.on_error = False
        self.omitted = [".cpp", ".c"]
        self.args = sys.argv  # get the args
        self.remove = False
        self.instr = []  # the instructions to be executed
        self.mk = True  # wether or not theres an -mk file
        self.flags = {"-i": self.makefile_f, "-r": self.push_remove, "-h": self.print_help}
        self.targets = []; 

#commands---------------------------------------------------------        
        
    def mk_target(self):
        mktgt = ""
        current_dir = os.popen("pwd").read().strip()
        files = os.listdir(current_dir)
        
        print("WARNING---------------------------------\n"
              +"pycma will try to compile the makefile\n"
              +"pycma is really fucking dumb so please\n"
              +"dont add anythin to the folder until compilation is done\n"
              +"---------------------------------------"
        )

        makefile_loop(self.targets[0])
        self.comp_this("")
        files2 = os.listdir(current_dir)
        for f in files2:
            if f not in files:
                mktgt = f

        if len(mktgt):
            self.targets.append(mktgt)
        else:
            print("it appears the file is already up to date\n")

            
    
#flag functions--------------------------------------------------

    def push_remove(self):
        self.remove = True

        
    def makefile_f(self):
        self.mk = False
        
    def print_help(self):
        print(
            "HELP--------------------------\n"
            + "-i ignores the make file\n"
            + "-r removes the executable after its done\n"
            + "-h prints this menu"
            + "\n------------------------------"
        )
        print("CURRENT STATE IS:")
        print("mk: ",  self.mk, ", targets: ", self.targets)
        print("args: ", self.args)
        print("instructs: ", self.instr)
        print("on error: ", self.on_error)

#other functions---------------------------------------------------
   
   #find if there's a makefile
    def find_mk(self):  # determine if mk is present
        mk_found = False
        current_dir = os.popen("pwd").read().strip()
        files = os.listdir(current_dir)
        if "makefile" in files:
            mk_found = True
        else:
            for fil in files:
                if fil[-3:] == ".mk":
                    mk_found = True
        self.mk = mk_found
                    

    #remove extensions
    def as_target(self,c):
        self.omitted.sort(key=lambda x: len(x), reverse=True)
        # try to avoid strings that contain another
        for ext in self.omitted:
            index = c.find(ext)
            while index > 0:
                c = c[:index] + c[index+len(ext):]
                index = c.find(ext)
                
        return c
        
    #do the things the flags say            
    def parse_flags(self):
        exec_stack = []
        for arg in self.args[1:]:
            if arg in self.flags:
                if arg == "-h":
                    self.instr.insert(0, self.flags[arg])
                else: 
                    exec_stack.append(self.flags[arg])
                    self.args.remove(arg)


        while(len(exec_stack)):
            func = exec_stack.pop()
            func()
            
#main execution
    def get_targets(self):
        for arg in self.args[1:]:
            if arg not in self.flags:
                self.targets.append(self.as_target(arg))
        print(self.targets)

        
    def push_execs(self):
        for target in self.targets[1:]:
            self.instr.append(basic_loop(target))
        print(self.instr)

            


            
a = pycma()
a.parse_flags()
if a.mk:
    a.find_mk()
    a.mk_target()
a.get_targets()
a.push_execs()
