using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.Linq;
using System.Management;
using System.Security.Principal;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;
using Microsoft.Win32;

namespace DefenderDisabler
{
    public partial class Form1 : Form
    {
        private readonly Timer checkTimer;
        private readonly bool isAdmin;

        public Form1()
        {
            InitializeComponent();

            // Check if running as administrator
            isAdmin = IsRunAsAdmin();

            // Initialize timer
            checkTimer = new Timer();
            checkTimer.Interval = 5000; // Check every 5 seconds
            checkTimer.Tick += CheckTimer_Tick;
        }

        private bool IsRunAsAdmin()
        {
            WindowsIdentity identity = WindowsIdentity.GetCurrent();
            WindowsPrincipal principal = new WindowsPrincipal(identity);
            return principal.IsInRole(WindowsBuiltInRole.Administrator);
        }

        private void CheckTimer_Tick(object sender, EventArgs e)
        {
            Task.Run(() => CheckAndDisableDefender());
        }

        private void CheckAndDisableDefender()
        {
            try
            {
                // Check if Real-time Protection is enabled
                bool isEnabled = IsRealTimeProtectionEnabled();

                if (isEnabled)
                {
                    // Disable Real-time Protection
                    DisableRealTimeProtection();

                    // Notify user
                    this.Invoke((MethodInvoker)delegate
                    {
                        notifyIcon1.BalloonTipTitle = "Defender Disabler";
                        notifyIcon1.BalloonTipText = "Real-time Protection has been disabled.";
                        notifyIcon1.BalloonTipIcon = ToolTipIcon.Info;
                        notifyIcon1.ShowBalloonTip(3000);
                    });
                }
            }
            catch (Exception ex)
            {
                this.Invoke((MethodInvoker)delegate
                {
                    notifyIcon1.BalloonTipTitle = "Defender Disabler - Error";
                    notifyIcon1.BalloonTipText = $"Error: {ex.Message}";
                    notifyIcon1.BalloonTipIcon = ToolTipIcon.Error;
                    notifyIcon1.ShowBalloonTip(3000);
                });
            }
        }

        private bool IsRealTimeProtectionEnabled()
        {
            try
            {
                // Use WMI to query Windows Defender settings
                ManagementScope scope = new ManagementScope(@"root\Microsoft\Windows\Defender");
                scope.Connect();

                ObjectQuery query = new ObjectQuery("SELECT * FROM MSFT_MpPreference");

                using (ManagementObjectSearcher searcher = new ManagementObjectSearcher(scope, query))
                {
                    foreach (ManagementObject obj in searcher.Get())
                    {
                        object disableRTM = obj["DisableRealtimeMonitoring"];
                        if (disableRTM != null)
                        {
                            bool isDisabled = (bool)disableRTM;
                            return !isDisabled;
                        }
                    }
                }
            }
            catch
            {
                // If we can't check, assume it's enabled to be safe
                return true;
            }

            return false;
        }

        private void DisableRealTimeProtection()
        {
            try
            {
                // Use WMI to call the Set method on MSFT_MpPreference
                ManagementScope scope = new ManagementScope(@"root\Microsoft\Windows\Defender");
                scope.Connect();

                ManagementPath path = new ManagementPath("MSFT_MpPreference");
                ManagementClass mpClass = new ManagementClass(scope, path, null);

                ManagementBaseObject inParams = mpClass.GetMethodParameters("Set");
                inParams["DisableRealtimeMonitoring"] = true;

                ManagementBaseObject outParams = mpClass.InvokeMethod("Set", inParams, null);

                if (outParams != null && outParams["ReturnValue"] != null)
                {
                    uint returnValue = (uint)outParams["ReturnValue"];
                    if (returnValue != 0)
                    {
                        throw new Exception($"WMI method returned error code: {returnValue}");
                    }
                }
            }
            catch (Exception ex)
            {
                throw new Exception($"Error disabling protection: {ex.Message}");
            }
        }

        private void exitToolStripMenuItem_Click(object sender, EventArgs e)
        {
            Application.Exit();
        }

        private void Form1_Load(object sender, EventArgs e)
        {
            if (!isAdmin)
            {
                MessageBox.Show(
                    "This application requires administrator privileges to run.\n\nPlease right-click the application and select 'Run as Administrator'.",
                    "Defender Disabler",
                    MessageBoxButtons.OK,
                    MessageBoxIcon.Error
                );

                Application.Exit();
                return;
            }

            this.WindowState = FormWindowState.Minimized;
            this.ShowInTaskbar = false;
            this.Hide();

            notifyIcon1.BalloonTipTitle = "Defender Disabler";
            notifyIcon1.BalloonTipText = "The application has started.";
            notifyIcon1.BalloonTipIcon = ToolTipIcon.Info;
            notifyIcon1.ShowBalloonTip(3000);

            // Start monitoring
            checkTimer.Start();
        }

        private void Form1_FormClosing(object sender, FormClosingEventArgs e)
        {
            checkTimer?.Stop();
            checkTimer?.Dispose();
            notifyIcon1.Visible = false;
            notifyIcon1.Dispose();
        }
    }
}
