import java.io.*;
import java.util.*;
import java.awt.*;
import java.awt.event.*;
import javax.swing.*;


public class PlotCommand {
  DoubleBufferedJPanel dbjp;
  Component f;

  PlotCommand(DoubleBufferedJPanel dbjp, Component f) {
    this.dbjp = dbjp;
    this.f = f;
  }

  public void doPlot(String command) {
    StringTokenizer st = new StringTokenizer(command);
    if (st.hasMoreTokens()) {
      String commandName = st.nextToken();
      if (commandName.equals("pause")) {
        JOptionPane.showConfirmDialog(f,"PAUSE","Go",
          JOptionPane.DEFAULT_OPTION,JOptionPane.INFORMATION_MESSAGE);
      }
      if (commandName.equals("clear")) {
        dbjp.clear();
      }
      if (commandName.equals("flush")) {
        dbjp.repaint();
      }
      if (commandName.equals("message")) {
        String message="";
        while(st.hasMoreTokens()) {
          message = message + " " + st.nextToken();
        }
        ((JLabel)f).setText(message);
        dbjp.repaint();
      }
      if (commandName.equals("red")||commandName.equals("green")||
        commandName.equals("blue")||commandName.equals("yellow")) 
      {
        dbjp.setColor(commandName);
      }
      if (commandName.equals("line")) {
        boolean argumentcheck = true;
        int x1=0;
        int x2=0;
        int y1=0;
        int y2=0;
        if (st.hasMoreTokens()) {
          x1 = (int)Double.parseDouble(st.nextToken());
        } else {
          argumentcheck = false;
        }
        if (st.hasMoreTokens()) {
          y1 = (int)Double.parseDouble(st.nextToken());
        } else {
          argumentcheck = false;
        }
        if (st.hasMoreTokens()) {
          x2 = (int)Double.parseDouble(st.nextToken());
        } else {
          argumentcheck = false;
        }
        if (st.hasMoreTokens()) {
          y2 = (int)Double.parseDouble(st.nextToken());
        } else {
          argumentcheck = false;
        }
        if (argumentcheck) {
          dbjp.drawLine(x1,y1,x2,y2);
        } else {
          System.err.println("PlotCommand.doPlot: insufficient arguments in line");
        }
      }
      if (commandName.equals("circle")||commandName.equals("fillCircle")||
          commandName.equals("square")||commandName.equals("fillSquare")) {
        int x = 0;
        int y = 0;
        boolean argumentcheck = true;
        if (st.hasMoreTokens()) {
          x = (int)Double.parseDouble(st.nextToken());
        } else {
          argumentcheck = false;
        }
        if (st.hasMoreTokens()) {
          y = (int)Double.parseDouble(st.nextToken());
        } else {
          argumentcheck = false;
        }
        if (argumentcheck) {
          dbjp.drawMark(commandName,x,y);
        } else {
          System.err.println("PlotCommand.doPlot: insufficient arguments in circle");
        }
      }
    }
  }

  public static void main(String args[]) {
    		JFrame jf = new JFrame();
		jf.setDefaultCloseOperation(JFrame.DISPOSE_ON_CLOSE);
		jf.addWindowListener(new WindowAdapter() {
			public void windowClosed(WindowEvent e) {
				System.exit(0);
			}
		});
		jf.setTitle("DoubleBufferedJPanel");
        
        DoubleBufferedJPanel dbjp = new DoubleBufferedJPanel();
	    dbjp.setMinimumSize(new Dimension(400,400));
        dbjp.setPreferredSize(new Dimension(400,400));
 
        jf.getContentPane().add(dbjp,BorderLayout.CENTER);
        jf.pack();
//		jf.setResizable(false);
		jf.setVisible(true);
  		dbjp.init();
        PlotCommand pc = new PlotCommand(dbjp,jf);
        pc.doPlot("white");
        pc.doPlot("line 0 0 400 400");
        pc.doPlot("red");
        pc.doPlot("circle 100 100");
        pc.doPlot("square 20 20");
        pc.doPlot("green");
        pc.doPlot("fillCircle 50 100");
        pc.doPlot("fillSquare 100 50");
        pc.doPlot("flush");
        pc.doPlot("pause");
        pc.doPlot("clear");
  }
}