using System;
using System.Collections;
using System.ComponentModel;
using System.Configuration.Install;
using System.IO;
using System.Diagnostics;
using Microsoft.Win32.TaskScheduler;

namespace DefenderDisabler
{
    [RunInstaller(true)]
    public class CustomInstaller : Installer
    {
        public CustomInstaller() : base()
        {
            // InitializeComponent();
        }

        // Add to startup using Task Scheduler
        public override void Install(IDictionary stateSaver)
        {
            base.Install(stateSaver);
            try
            {
                string targetDir = Context.Parameters["targetdir"];
                string exePath = Path.Combine(targetDir, "DefenderDisabler.exe");

                using (TaskService ts = new TaskService())
                {
                    TaskDefinition td = ts.NewTask();
                    td.RegistrationInfo.Description = "by alperdev";
                    td.Principal.RunLevel = TaskRunLevel.Highest; // Administrator
                    td.Triggers.Add(new LogonTrigger());
                    td.Actions.Add(new ExecAction(exePath));

                    ts.RootFolder.RegisterTaskDefinition(@"DefenderDisablerTask", td);
                }
            }
            catch (Exception ex)
            {
                System.Windows.Forms.MessageBox.Show("Failed to add Defender Disabler to startup: " + ex.Message);
            }
        }

        // Remove from startup
        public override void Uninstall(IDictionary savedState)
        {
            base.Uninstall(savedState);
            try
            {
                using (TaskService ts = new TaskService())
                {
                    ts.RootFolder.DeleteTask("DefenderDisablerTask");
                }
            }
            catch (Exception)
            {
                // Ignore
            }
        }

        // Launch the .exe after installation
        protected override void OnAfterInstall(IDictionary savedState)
        {
            base.OnAfterInstall(savedState);
            try
            {
                string targetDir = Context.Parameters["targetdir"];
                string exePath = Path.Combine(targetDir, "DefenderDisabler.exe");

                Process.Start(exePath);
            }
            catch
            {
                // Ignore
            }
        }
    }
}