'''
Building a simple blockchain 
'''

import hashlib


'''
Generating block in info 
'''

def block(info:str):
    h = hashlib.sha256()
    h.update(info.encode('utf-8'))
    return h.hexdigest()


print(block("shubham"))    
    










