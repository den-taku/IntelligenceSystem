import java.io.*;
import java.util.*;
import java.awt.*;
import java.awt.event.*;
import javax.swing.*;

public class SimplePlotter {
  public static void main(String args[]) {
    		JFrame jf = new JFrame();
		jf.setDefaultCloseOperation(JFrame.DISPOSE_ON_CLOSE);
		jf.addWindowListener(new WindowAdapter() {
			public void windowClosed(WindowEvent e) {
				System.exit(0);
			}
		});
		jf.setTitle("SimplePlotter");
        JMenuBar mb = new JMenuBar();
        jf.setJMenuBar(mb);
        JMenu fileMenu = new JMenu("File");
        mb.add(fileMenu);
        JMenuItem exitItem = new JMenuItem("exit");
        fileMenu.add(exitItem);
        ActionListener exitlistner = new ActionListener()
        {
          public void actionPerformed(ActionEvent e)
          { System.exit(0); }
        };
        exitItem.addActionListener(exitlistner);
        
        JLabel jl=new JLabel("Simple Plotter");
        jf.getContentPane().add(jl,BorderLayout.SOUTH);
        DoubleBufferedJPanel dbjp = new DoubleBufferedJPanel();
	    dbjp.setMinimumSize(new Dimension(400,400));
        dbjp.setPreferredSize(new Dimension(400,400));
 
        jf.getContentPane().add(dbjp,BorderLayout.CENTER);
        jf.pack();
//		jf.setResizable(false);
		jf.setVisible(true);
  		dbjp.init();
        PlotCommand pc = new PlotCommand(dbjp,jl);
        pc.doPlot("white");
        
        InputStreamReader isr;
        BufferedReader br;
        isr = new InputStreamReader(System.in);
        br = new BufferedReader(isr);
        while(true) {
          try {
            String command = br.readLine();
            if (command == null) {
               break;
            }
            pc.doPlot(command);
            dbjp.repaint();
          }
          catch (IOException e) {
          }
        }
  }
}