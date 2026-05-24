import sys, hid
 
def list_devices():
    print("VID:PID\t\tusage_page\tusage\t\tmanufacturer / product")
    for d in hid.enumerate():
        print("{:04x}:{:04x}\tup={:#06x}\tu={:#06x}\t{} / {}".format(
            d["vendor_id"], d["product_id"],
            d["usage_page"], d["usage"],
            d.get("manufacturer_string") or "Unknown",
            d.get("product_string") or "Unknown"))

def dump(vid, pid):
    dev = hid.device()
    dev.open(vid, pid)
    prev = None

    try:
        while True:
            data = dev.read(64)
            
            # Print report only if it differs from the previous one
            if data and data != prev:
                print("[{:2}] {}".format(len(data), " ".join(f"{b:02x}" for b in data)))
                prev = data
    finally:
        dev.close()

if __name__ == "__main__":
    if len(sys.argv) < 3:
        list_devices()
        print("\nUsage: python dump.py <vid_hex> <pid_hex>")
    else:
        dump(int(sys.argv[1], 16), int(sys.argv[2], 16))