/*
    DoubleBufferedJPanel.java
*/

import java.awt.*;
import java.awt.event.*;
import javax.swing.*;

public class DoubleBufferedJPanel extends JPanel{
	Color blue = new Color(0,0,255);
	Color green = new Color(0,255,0);
    int colorToggle = 0;
	/* ダブルバッファリング用 */
	Image imgOffscreen;
	Graphics gOffscreen;
	Dimension d;
	
    DoubleBufferedJPanel () {
      setBackground(Color.white);
    }
	public void init(){
		/* オフスクリーン・バッファの用意 */
		d = getSize();
		imgOffscreen = createImage(d.width,d.height);
		gOffscreen = imgOffscreen.getGraphics();
		clear();
    }

    public void clear() {
		gOffscreen.setColor(Color.black);
		gOffscreen.fillRect(0,0,d.width,d.height);
		gOffscreen.setColor(Color.white);
        repaint();
    }

    public void drawLine(int x1, int y1, int x2, int y2) {
      gOffscreen.drawLine(x1,y1,x2,y2);
    }

    public void drawMark(String markType, int x, int y) {
      if (markType.equals("circle")) {
        drawCircle(x,y);
      }
      if (markType.equals("fillCircle")) {
        drawFillCircle(x,y);
      }
      if (markType.equals("square")) {
        drawSquare(x,y);
      }
      if (markType.equals("fillSquare")) {
        drawFillSquare(x,y);
      }
    }

    public void drawCircle(int x, int y) {
        gOffscreen.drawOval(x-2,y-2,4,4);
    }

    public void drawFillCircle(int x, int y) {
      gOffscreen.fillOval(x-2,y-2,4,4);
    }

    public void drawSquare(int x, int y) {
      gOffscreen.drawRect(x-2,y-2,4,4);
    }

    public void drawFillSquare(int x, int y) {
      gOffscreen.fillRect(x-2,y-2,4,4);
    }

    public void setColor(String colorname) {
      Color color = Color.white;
      if (colorname.equals("black")) {
        color=Color.black;
      }
      if (colorname.equals("white")) {
        color=Color.white;
      }
      if (colorname.equals("yellow")) {
        color=Color.yellow;
      }
      if (colorname.equals("red")) {
        color=Color.red;
      }
      if (colorname.equals("green")) {
        color=Color.green;
      }
      if (colorname.equals("blue")) {
        color=Color.blue;
      }
      gOffscreen.setColor(color);
    }

	public void update(Graphics g){
		paint(g);
	}
    public void paint(Graphics g){
        super.paint(g);
		if(imgOffscreen != null){
			g.drawImage(imgOffscreen,0,0,this);
		}
	}

	public static void main(String args[]){
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
		jf.setVisible(true);
  		dbjp.init();
        String colors[] = {"white","black","blue","red","green","yellow"};
        for (int i=0;i<=400;i+=10) {
          dbjp.setColor(colors[(i/10)%(colors.length)]);
          dbjp.drawLine(0,400,i,i);
          dbjp.drawCircle(i,i);
          dbjp.drawFillCircle(i,i+4);
          dbjp.drawSquare(i,i-4);
          dbjp.drawFillSquare(i-4,i);
        }
        dbjp.repaint();
	}
}
