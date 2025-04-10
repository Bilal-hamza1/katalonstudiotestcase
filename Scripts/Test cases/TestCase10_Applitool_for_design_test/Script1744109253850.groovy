import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.keyword.applitools.BasicKeywords as BasicKeywords

// 1. Initialize Applitools
BasicKeywords.eyesInit('97Be100GZNntHoyFDZ5J9CakKgrgizwOkXZiIRDXpJkKlo110' // Replace with your actual API key
    )

// 2. Open browser and navigate
WebUI.openBrowser('')

WebUI.navigateToUrl('https://applitools.com/helloworld2/')

// 3. First visual test
WebUI.click(findTestObject('Object Repository/Page_Applitools/a_diff1'))

BasicKeywords.checkWindow('Test1')

// 4. Second visual test
WebUI.click(findTestObject('Object Repository/Page_Applitools/a_diff2'))

WebUI.click(findTestObject('Object Repository/Page_Applitools/button_Click me'))

BasicKeywords.checkWindow('Test2')

// 5. Clean up
WebUI.closeBrowser()

