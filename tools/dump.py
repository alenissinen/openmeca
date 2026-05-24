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

def read_feature(vid, pid, report_id=0, length=65):
    dev = hid.device()
    dev.open(vid, pid)

    try:
        data = dev.get_feature_report(report_id, length)
        print("[{:2}] {}".format(len(data), " ".join(f"{b:02x}" for b in data)))
    finally:
        dev.close()

if __name__ == "__main__":
    print("Usage:\n\tpython dump.py <vid_hex> <pid_hex>\n\tpython dump.py feature <vid_hex> <pid_hex>\n")
    if len(sys.argv) < 3:
        list_devices()
    elif sys.argv[1] == "feature":
        read_feature(int(sys.argv[2], 16), int(sys.argv[3], 16))
    else:
        dump(int(sys.argv[1], 16), int(sys.argv[2], 16))