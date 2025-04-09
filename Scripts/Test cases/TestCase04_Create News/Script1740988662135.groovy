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

WebUI.openBrowser('')

WebUI.navigateToUrl('https://testing.aljomaihfamilyapp.com/jutghmk/auth/login')

WebUI.setText(findTestObject('Object Repository/Page_/input__username'), 'superadmin')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_/input__password'), '373N5QL8KVQ=')

WebUI.click(findTestObject('Object Repository/Page_/button_'))

WebUI.click(findTestObject('Object Repository/Page_/span_'))

WebUI.click(findTestObject('Object Repository/Page_/span_14'))

WebUI.click(findTestObject('Object Repository/Page_/span_'))

WebUI.click(findTestObject('Object Repository/Page_/span_44'))

WebUI.click(findTestObject('Object Repository/Page_/a_'))

WebUI.click(findTestObject('Object Repository/Page_/button__1'))

WebUI.click(findTestObject('Object Repository/Page_/i__fa fa-plus'))

WebUI.click(findTestObject('Object Repository/Page_/button__1_2'))

WebUI.click(findTestObject('Object Repository/Page_/i__fa fa-plus_1'))

WebUI.click(findTestObject('Object Repository/Page_/input__btn-submit'))

WebUI.setText(findTestObject('Object Repository/Page_/input__baby_name'), 'baby name')

WebUI.click(findTestObject('Object Repository/Page_/input__btn-submit'))

WebUI.click(findTestObject('Object Repository/Page_/a__1'))

WebUI.click(findTestObject('Object Repository/Page_/a__1_2'))

WebUI.closeBrowser()

