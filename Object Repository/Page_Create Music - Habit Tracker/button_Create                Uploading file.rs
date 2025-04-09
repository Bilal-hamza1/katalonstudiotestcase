<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>button_Create                Uploading file</name>
   <tag></tag>
   <elementGuidId>4f9b43ce-1f5d-4d76-9ba8-072fe621c99d</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>#key-bindings-1</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//button[@id='key-bindings-1']</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>internal:role=button[name=&quot;Create&quot;i]</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>button</value>
      <webElementGuid>da357436-54a5-469a-9660-f06c227de62c</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-bind:id</name>
      <type>Main</type>
      <value>$id('key-bindings')</value>
      <webElementGuid>d9782242-35d3-4cfd-a9a2-739eb85bba30</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-mousetrap.global.mod-s</name>
      <type>Main</type>
      <value>document.getElementById($el.id).click()</value>
      <webElementGuid>ff7e3e73-5d47-44ed-8ba6-48e6954e02e9</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-data</name>
      <type>Main</type>
      <value>{
            form: null,
            isProcessing: false,
            processingMessage: null,
        }</value>
      <webElementGuid>a5742de8-8eee-43b8-8f79-afa19b7c07b4</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-init</name>
      <type>Main</type>
      <value>
            form = $el.closest('form')

            form?.addEventListener('form-processing-started', (event) => {
                isProcessing = true
                processingMessage = event.detail.message
            })

            form?.addEventListener('form-processing-finished', () => {
                isProcessing = false
            })
        </value>
      <webElementGuid>de1bdbec-c20b-4774-838c-cdf4dd47f5b0</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-bind:class</name>
      <type>Main</type>
      <value>{ 'enabled:opacity-70 enabled:cursor-wait': isProcessing }</value>
      <webElementGuid>2f9c16eb-5b82-44b8-a389-7902f06efd16</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>fi-btn relative grid-flow-col items-center justify-center font-semibold outline-none transition duration-75 focus-visible:ring-2 rounded-lg fi-color-custom fi-btn-color-primary fi-color-primary fi-size-md fi-btn-size-md gap-1.5 px-3 py-2 text-sm inline-grid shadow-sm bg-custom-600 text-white hover:bg-custom-500 focus-visible:ring-custom-500/50 dark:bg-custom-500 dark:hover:bg-custom-400 dark:focus-visible:ring-custom-400/50 fi-ac-action fi-ac-btn-action</value>
      <webElementGuid>e6e7427a-996c-46c3-876d-74e8f13bb976</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>type</name>
      <type>Main</type>
      <value>submit</value>
      <webElementGuid>f482fdb6-72ea-4db9-9578-f73701ce5e16</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>wire:loading.attr</name>
      <type>Main</type>
      <value>disabled</value>
      <webElementGuid>96e766bc-e5e1-491c-8fe4-73aa5b42c441</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-bind:disabled</name>
      <type>Main</type>
      <value>isProcessing</value>
      <webElementGuid>711194b3-ada3-47d3-9e2a-d79cec4996fa</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>key-bindings-1</value>
      <webElementGuid>2ba4fa0b-fc89-408c-83c7-280f1fb641b6</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>data-has-alpine-state</name>
      <type>Main</type>
      <value>true</value>
      <webElementGuid>f5a1745f-5959-47c3-9907-dc0c2357a651</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
            

                    
    
    

        

                    
    
    

        
    

    
        Create
    

            Uploading file...
    

    

    
</value>
      <webElementGuid>ec6d502e-c85a-475d-b7cf-458403c674b6</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;key-bindings-1&quot;)</value>
      <webElementGuid>fdd30005-50ba-4172-a236-911371d123f0</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//button[@id='key-bindings-1']</value>
      <webElementGuid>3521e72a-409c-4270-9bf4-52fb2b5996b3</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//form[@id='form']/div[2]/div/button</value>
      <webElementGuid>7861d29c-00cb-4052-857b-806179e98309</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Files'])[2]/following::button[1]</value>
      <webElementGuid>222e0476-c198-4e9d-ac35-f3471c62938f</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Frame 6095.png Upload complete'])[1]/following::button[1]</value>
      <webElementGuid>6a1ae98f-a06f-4e95-a476-b4abf2ba1c99</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[2]/div/button</value>
      <webElementGuid>21745e88-812e-413e-ab1d-1231328f466e</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//button[@type = 'submit' and @id = 'key-bindings-1' and (text() = '
            

                    
    
    

        

                    
    
    

        
    

    
        Create
    

            Uploading file...
    

    

    
' or . = '
            

                    
    
    

        

                    
    
    

        
    

    
        Create
    

            Uploading file...
    

    

    
')]</value>
      <webElementGuid>bc2d8046-0340-403c-9ab0-c0adad30b640</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
