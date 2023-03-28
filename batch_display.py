import sys
from joblib import Parallel, delayed
import display

def async_display(input,desc):
  #Does some work which takes a while
  s = "display.py " + input + " " + input[:-3] + "mp4"
  #print(s)
  display.run(input, input[:-3] + "mp4",desc)


def draw_plots():
  inputs = sys.argv[1:]
  #for i in inputs:
  print("Start")
  Parallel(n_jobs=4)(delayed(async_display)(inputs[i],i) for i in range(len(inputs)))
  print("End.")
  
if __name__ == "__main__":
    draw_plots()