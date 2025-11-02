using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace DefenderDisabler
{
    internal static class Program
    {
        private static Mutex mutex = null;

        /// <summary>
        /// The main entry point for the application.
        /// </summary>
        [STAThread]
        static void Main()
        {
            const string mutexName = "DefenderDisablerSingleInstanceMutex";
            bool createdNew;

            mutex = new Mutex(true, mutexName, out createdNew);

            if (!createdNew)
            {
                MessageBox.Show(
                    "Defender Disabler is already running.\n\nCheck the system tray for the application icon.",
                    "Already Running",
                    MessageBoxButtons.OK,
                    MessageBoxIcon.Exclamation
                );
                return;
            }

            try
            {
                Application.EnableVisualStyles();
                Application.SetCompatibleTextRenderingDefault(false);
                Application.Run(new Form1());
            }
            finally
            {
                if (mutex != null)
                {
                    mutex.ReleaseMutex();
                    mutex.Dispose();
                }
            }
        }
    }
}
