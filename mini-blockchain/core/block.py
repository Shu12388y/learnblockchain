class Block:
    def __init__(self,blockheader,prevhash,data,tx):
        self.blockheader = blockheader
        self.prevhash = prevhash
        self.data = data
        self.tx = tx
        self.blockCount = 0
